use diesel_full_text_search::TsVectorExtensions;
use crate::users::ResponseUser;
use futures_concurrency::future::TryJoin;
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::diesel::deserialize::Queryable;
use rocket_db_pools::Connection;
use crate::error::{DeductError, DeductResult};
use crate::schema::*;
use crate::model::*;
use crate::search::{SearchResultGraph, TrendingRange};
use crate::api::users::AuthenticatedUser;
use crate::model::objective::{ResponseObjPrerequisite, ObjectivePrerequisite};

#[derive(Debug, Serialize, Deserialize, Associations, Selectable, Queryable, Insertable, Identifiable)]
#[diesel(table_name = knowledge_graphs, belongs_to(User, foreign_key = author))]
pub struct KnowledgeGraph {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub author: i64,
    pub last_modified: chrono::NaiveDate,
    pub like_count: i32
}

/// A full response to the user that provides all information necessary to render a graph and institute all
/// constraints, such as edges within the graph to structure progress, and requirements outside the graph
/// to indicate a necessary piece of prior knowledge.
#[derive(Serialize)]
pub struct ResponseGraph {
    #[serde(flatten)]
    pub graph: KnowledgeGraph,
    pub topics: Vec<Topic>,
    pub requirements: Vec<(i64, i64)>,
    pub objectives: Vec<ResponseObjPrerequisite>,
    pub progress: Vec<i64>
}

#[derive(Serialize)]
pub struct PreviewGraph {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String
}

diesel::allow_columns_to_appear_in_same_group_by_clause!(users::username, users::avatar, knowledge_graphs::id, knowledge_graphs::like_count,
    knowledge_graphs::last_modified, knowledge_graphs::author, knowledge_graphs::description, knowledge_graphs::name);

impl KnowledgeGraph {
    pub async fn create(user_id: i64, name: String, description: String, conn: &mut Connection<Db>) -> DeductResult<KnowledgeGraph> {
        Ok(diesel::insert_into(knowledge_graphs::table)
            .values((knowledge_graphs::id.eq(uuid::Uuid::new_v4()), knowledge_graphs::author.eq(user_id), knowledge_graphs::name.eq(name), knowledge_graphs::description.eq(description)))
            .returning(KnowledgeGraph::as_select())
            .get_result(conn)
            .await?)
    }

    pub async fn get(id: uuid::Uuid, conn: &mut Connection<Db>) -> DeductResult<KnowledgeGraph> {
        Ok(knowledge_graphs::table
            .filter(knowledge_graphs::id.eq(id))
            .select(KnowledgeGraph::as_select())
            .first::<KnowledgeGraph>(conn)
            .await?)
    }

    pub async fn get_from_path(username: String, title: String, conn: &mut Connection<Db>) -> DeductResult<KnowledgeGraph> {
        let user = User::get_from_username(username, conn).await?;

        Ok(knowledge_graphs::table
            .filter(
                knowledge_graphs::author.eq(user.id)
                .and(knowledge_graphs::name.eq(title))
                )
            .select(KnowledgeGraph::as_select())
            .first::<KnowledgeGraph>(conn)
            .await?)
    }

    pub async fn delete(id: uuid::Uuid, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(knowledge_graphs::table.filter(knowledge_graphs::id.eq(id)))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn delete_topic(id: uuid::Uuid, topic_id: i64, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(
            topics::table.filter(
                topics::id.eq(topic_id)
                .and(topics::knowledge_graph_id.eq(id)))
            )
            .execute(conn)
            .await?;
    
        Ok(())
    }

    pub async fn delete_requirement(id: uuid::Uuid, req: (i64, i64), conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(
            requirements::table.filter(
                requirements::source.eq(req.0)
                .and(requirements::destination.eq(req.1))
                .and(requirements::knowledge_graph_id.eq(id)))
            )
            .execute(conn)
            .await?;
    
        Ok(())
    }

    pub async fn delete_satisfier(id: uuid::Uuid, topic: i64, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(objective_satisfiers::table
            .filter(objective_satisfiers::knowledge_graph_id.eq(id)
                .and(objective_satisfiers::topic.eq(topic))))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn delete_prerequisite(id: uuid::Uuid, source_topic: i64, dest_topic: i64, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(objective_prerequisites::table
            .filter(objective_prerequisites::knowledge_graph_id.eq(id)
                .and(objective_prerequisites::topic.eq(source_topic))
                .and(objective_prerequisites::suggested_topic.eq(dest_topic))))
            .execute(conn)
            .await?;

        Ok(())
    }
    
    pub async fn to_response(self, maybe_user: Option<AuthenticatedUser>, conn: &mut Connection<Db>) -> DeductResult<ResponseGraph> {
        let topics_query = Topic::belonging_to(&self)
            .load::<Topic>(conn);

        let requirements_query = Requirement::belonging_to(&self)
            .load::<Requirement>(conn);

        let (topics, requirements) = (topics_query, requirements_query).try_join().await?;

        let (objectives, progress) = match maybe_user {
            Some(user) => 
                (objective_prerequisites::table
                    .inner_join(objectives::table)
                    .inner_join(knowledge_graphs::table.on(objective_prerequisites::suggested_graph.eq(knowledge_graphs::id)))
                    .left_join(user_objective_progress::table.on(
                        user_objective_progress::objective_id.eq(objective_prerequisites::objective)
                        .and(user_objective_progress::user_id.eq(user.db_id))
                    ))
                    .select((ObjectivePrerequisite::as_select(), Objective::as_select(), KnowledgeGraph::as_select(), user_objective_progress::user_id.nullable()))
                    .filter(objective_prerequisites::knowledge_graph_id.eq(self.id))
                    .load::<(ObjectivePrerequisite, Objective, KnowledgeGraph, Option<i64>)>(conn).await?,

                Progress::get_user_progress(user.db_id, self.id, conn).await?),

            None => (Vec::with_capacity(0), Vec::with_capacity(0))
        };

        Ok(ResponseGraph {
            graph: self,

            topics: topics,

            requirements: requirements
                .into_iter()
                .map(|req| (req.source, req.destination))
                .collect(),

            objectives: objectives
                .into_iter()
                .map(|(prereq, obj, suggested_graph, satisfied)| ResponseObjPrerequisite {
                    knowledge_graph_id: prereq.knowledge_graph_id,
                    topic: prereq.topic,
                    objective: obj,
                    satisfied: satisfied.is_some(),
                    suggested_topic: prereq.suggested_topic,
                    suggested_graph: PreviewGraph {
                        id: suggested_graph.id,
                        name: suggested_graph.name,
                        description: suggested_graph.description
                    }
                })
                .collect(),

            progress: progress
                .into_iter()
                .map(|p| p.topic)
                .collect()
        })
    }

    pub async fn search(query: String, offset: i64, maybe_user: Option<AuthenticatedUser>, conn: &mut Connection<Db>) -> DeductResult<Vec<SearchResultGraph>> {
        let intermediate = knowledge_graphs::table
            .inner_join(users::table)
            .filter(knowledge_graphs::tsv_name_desc.matches(diesel_full_text_search::websearch_to_tsquery(query)))
            .select((KnowledgeGraph::as_select(), User::as_select()))
            .offset(offset * 10)
            .limit(10)
            .load::<(KnowledgeGraph, User)>(conn)
            .await?;

        SearchResultGraph::get_likes(intermediate, maybe_user, conn).await
    }

    pub async fn update_info(id: uuid::Uuid, title: String, description: String, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::update(knowledge_graphs::table)
            .filter(knowledge_graphs::id.eq(id))
            .set((knowledge_graphs::name.eq(title), knowledge_graphs::description.eq(description)))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn trending(range: TrendingRange, limit: i64, maybe_user: Option<AuthenticatedUser>, conn: &mut Connection<Db>) -> DeductResult<Vec<SearchResultGraph>> {
        use diesel::dsl::*;

        let days = match range {
            TrendingRange::Day => 1,
            TrendingRange::Week => 7,
            TrendingRange::Month => 31,
            TrendingRange::AllTime => 100000
        };

        
        let intermediate = likes::table
            .filter(likes::like_date.ge(date(now - days.days())))
            .inner_join(knowledge_graphs::table
                .inner_join(users::table)
            )
            .group_by((knowledge_graphs::id, users::username, users::avatar))
            .select((count_star(), (KnowledgeGraph::as_select(), users::username, users::avatar)))
            .limit(limit)
            .order(count_star().desc())
            .load::<(i64, (KnowledgeGraph, String, Option<String>))>(conn)
            .await?
            .into_iter()
            .map(|(count, (graph, username, avatar))| {
                (KnowledgeGraph {
                    like_count: count as i32,
                    ..graph
                },
                ResponseUser {
                    username: username,
                    avatar: avatar
                })
            })
            .collect();

        SearchResultGraph::get_likes(intermediate, maybe_user, conn).await
    }
}

/// Represents an incoming request to create a `KnowledgeGraph`.
#[derive(Deserialize, FromForm)]
pub struct KnowledgeGraphCreation {
    pub name: String,
    pub description: String
}
