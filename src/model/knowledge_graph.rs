use rocket_db_pools::diesel::deserialize::FromSqlRow;
use crate::diesel_full_text_search::TsVectorExtensions;
use crate::users::ResponseUser;
use futures_concurrency::future::TryJoin;
use diesel::pg::Pg;
use rocket_db_pools::diesel::row::Row;
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::diesel::deserialize::{self, Queryable};
use rocket_db_pools::Connection;
use crate::error::{DeductError, DeductResult};
use crate::schema::*;
use crate::model::*;
use crate::search::{SearchResultGraph, TrendingRange};
use crate::api::users::AuthenticatedUser;

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

// ABANDON ALL HOPE, YE WHO ENTER HERE
// DIESEL HAS BROKE MY HEART AND FORCED ME TO CIRCUMVENT ITS BEAUTY
// THERE IS NO WARRANTY AND NO HOPE BEYOND THIS POINT
struct InternalTsvector;

struct InternalKnowledgeGraph {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub author: i64,
    pub last_modified: chrono::NaiveDate,
    pub tsv_name_desc: InternalTsvector,
    pub like_count: i32
}

use diesel::sql_types::*;

type KnowledgeGraphReturning = (Uuid, Text, Text, BigInt, Date, diesel_full_text_search::TsVector, Integer);

impl Into<InternalKnowledgeGraph> for KnowledgeGraph {
    fn into(self) -> InternalKnowledgeGraph {
        InternalKnowledgeGraph {
            id: self.id,
            name: self.name,
            description: self.description,
            author: self.author,
            last_modified: self.last_modified,
            tsv_name_desc: InternalTsvector {},
            like_count: self.like_count
        }
    }
}

// This is not updated when schemas are updated, must be done, or risk runtime crash(?)
impl FromSqlRow<KnowledgeGraphReturning, Pg> for InternalKnowledgeGraph {
    fn build_from_row<'a>(row: &impl Row<'a, Pg>) -> deserialize::Result<Self> {

        Ok(InternalKnowledgeGraph {
            id: row.get_value::<Uuid, uuid::Uuid, usize>(0)?,
            name: row.get_value::<Text, String, usize>(1)?,
            description: row.get_value::<Text, String, usize>(2)?,
            author: row.get_value::<BigInt, i64, usize>(3)?,
            last_modified: row.get_value::<Date, chrono::NaiveDate, usize>(4)?,
            tsv_name_desc: InternalTsvector {},
            like_count: row.get_value::<Integer, i32, usize>(6)?
        })

    }
}

const KG_SELECT: (knowledge_graphs::columns::id, knowledge_graphs::columns::name, knowledge_graphs::columns::description, 
    knowledge_graphs::columns::author, knowledge_graphs::columns::last_modified, knowledge_graphs::columns::like_count)
 = (knowledge_graphs::id, knowledge_graphs::name,  knowledge_graphs::description, knowledge_graphs::author, knowledge_graphs::last_modified, knowledge_graphs::like_count);

/// A full response to the user that provides all information necessary to render a graph and institute all
/// constraints, such as edges within the graph to structure progress, and requirements outside the graph
/// to indicate a necessary piece of prior knowledge.
#[derive(Serialize)]
pub struct ResponseGraph {
    #[serde(flatten)]
    pub graph: KnowledgeGraph,
    pub topics: Vec<Topic>,
    pub requirements: Vec<(i64, i64)>,
    pub objectives: Vec<(i64, Objective)>,
    pub progress: Option<Vec<i64>>
}

impl KnowledgeGraph {
    pub async fn create(user_id: i64, name: String, description: String, conn: &mut Connection<Db>) -> DeductResult<KnowledgeGraph> {
        Ok(diesel::insert_into(knowledge_graphs::table)
            .values((knowledge_graphs::id.eq(uuid::Uuid::new_v4()), knowledge_graphs::author.eq(user_id), knowledge_graphs::name.eq(name), knowledge_graphs::description.eq(description)))
            .get_result(conn)
            .await
            .map(|x: InternalKnowledgeGraph| -> KnowledgeGraph {
                KnowledgeGraph {
                    id: x.id,
                    name: x.name,
                    description: x.description,
                    author: x.author,
                    last_modified: x.last_modified,
                    like_count: x.like_count
                }
            })?)
    }

    pub async fn get(id: uuid::Uuid, conn: &mut Connection<Db>) -> DeductResult<KnowledgeGraph> {
        Ok(knowledge_graphs::table
            .filter(knowledge_graphs::id.eq(id))
            .select(KG_SELECT)
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
            .select(KG_SELECT)
            .first::<KnowledgeGraph>(conn)
            .await?)
    }

    #[inline(always)]
    pub fn check_owner(&self, id: i64) -> DeductResult<()> {
        if self.author == id {
            Ok(())
        } else {
            Err(DeductError::UnauthorizedUser("User is not authorized to access this graph".to_string()))
        }
    }

    pub async fn delete(self, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(knowledge_graphs::table.filter(knowledge_graphs::id.eq(self.id)))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn delete_topic(&self, topic_id: i64, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(
            topics::table.filter(
                topics::id.eq(topic_id)
                .and(topics::knowledge_graph_id.eq(self.id)))
            )
            .execute(conn)
            .await?;
    
        Ok(())
    }

    pub async fn delete_requirement(&self, req: (i64, i64), conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(
            requirements::table.filter(
                requirements::source.eq(req.0)
                .and(requirements::destination.eq(req.1))
                .and(requirements::knowledge_graph_id.eq(self.id)))
            )
            .execute(conn)
            .await?;
    
        Ok(())
    }
    
    pub async fn to_response(self, conn: &mut Connection<Db>) -> DeductResult<ResponseGraph> {
        let topics_query = Topic::belonging_to(&self)
            .load::<Topic>(conn);

        let requirements_query = Requirement::belonging_to(&self)
            .load::<Requirement>(conn);

        let obj_pre_query = objective_prerequisites::table
            .inner_join(objectives::table)
            .select((ObjectivePrerequisite::as_select(), Objective::as_select()))
            .filter(objective_prerequisites::knowledge_graph_id.eq(self.id))
            .load::<(ObjectivePrerequisite, Objective)>(conn);

        let (topics, requirements, objectives) = (topics_query, requirements_query, obj_pre_query).try_join().await?;

        Ok(ResponseGraph {
            graph: self,

            topics: topics,

            requirements: requirements
                .into_iter()
                .map(|req| (req.source, req.destination))
                .collect(),

            objectives: objectives
                .into_iter()
                .map(|(prereq, obj)| (prereq.topic, obj))
                .collect(),

            progress: None
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

    pub async fn update_info(self, title: String, description: String, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::update(knowledge_graphs::table)
            .filter(knowledge_graphs::id.eq(self.id))
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

        diesel::allow_columns_to_appear_in_same_group_by_clause!(users::username, users::avatar, knowledge_graphs::id, knowledge_graphs::like_count,
            knowledge_graphs::last_modified, knowledge_graphs::author, knowledge_graphs::description, knowledge_graphs::name);

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
