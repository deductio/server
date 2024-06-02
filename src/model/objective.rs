use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use crate::error::DeductResult;
use crate::schema::*;
use crate::model::{Db, KnowledgeGraph, User};
use crate::search::SearchResultGraph;
use crate::users::AuthenticatedUser;
use diesel_full_text_search::TsVectorExtensions;
use crate::model::knowledge_graph::PreviewGraph;

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable, Clone)]
#[diesel(table_name = objectives)]
pub struct Objective {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    pub title: String,
    pub description: String
}

impl Objective {
    pub async fn get(id: i64, mut conn: Connection<Db>) -> DeductResult<Objective> {
        Ok(objectives::table.filter(objectives::id.eq(id)).select(Objective::as_select()).first(&mut conn).await?)
    }

    pub async fn create(user: AuthenticatedUser, title: String, description: String, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::insert_into(objectives::table)
            .values((
                objectives::title.eq(title), 
                objectives::description.eq(description),
                objectives::author.eq(user.db_id)
            ))
            .execute(conn).await?;

        Ok(())
    }

    pub async fn get_satisfied_graphs(id: i64, page: i64, conn: &mut Connection<Db>) -> DeductResult<Vec<SearchResultGraph>> {
        let res = objective_satisfiers::table
            .filter(objective_satisfiers::objective.eq(id))
            .inner_join(
                knowledge_graphs::table
                    .on(objective_satisfiers::knowledge_graph_id.eq(knowledge_graphs::id))
                    .inner_join(users::table)
            )
            .select((KnowledgeGraph::as_select(), User::as_select()))
            .distinct()
            .limit(10)
            .offset(page * 10)
            .load::<(KnowledgeGraph, User)>(conn)
            .await?;

        SearchResultGraph::get_likes(res, None, conn).await  
    }

    pub async fn search_objectives(query: String, page: i64, conn: &mut Connection<Db>) -> DeductResult<Vec<Objective>> {
        Ok(objectives::table
            .filter(objectives::tsv_title_desc.matches(diesel_full_text_search::websearch_to_tsquery(query)))
            .limit(10)
            .offset(page * 10)
            .select(Objective::as_select())
            .load::<Objective>(conn)
            .await?)
    }
}

#[derive(Debug, Deserialize, Queryable, Insertable, Clone, Identifiable, Associations, Selectable, FromForm)]
#[diesel(table_name = objective_prerequisites, belongs_to(KnowledgeGraph), primary_key(knowledge_graph_id, topic, objective))]
pub struct ObjectivePrerequisite {
    pub knowledge_graph_id: uuid::Uuid,
    pub topic: i64,
    pub objective: i64,
    pub suggested_topic: i64,
    pub suggested_graph: uuid::Uuid
}

impl ObjectivePrerequisite {
    pub async fn commit(&self, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::insert_into(objective_prerequisites::table)
            .values(self)
            .execute(conn)
            .await?;

        Ok(())
    }
}

#[derive(Serialize)]
pub struct ResponseObjPrerequisite {
    pub knowledge_graph_id: uuid::Uuid,
    pub topic: i64,
    pub objective: Objective,
    pub suggested_topic: i64,
    pub suggested_graph: PreviewGraph,
    pub satisfied: bool
}

#[derive(Insertable, Queryable, FromForm, Serialize)]
pub struct ObjectiveSatisfier {
    pub knowledge_graph_id: uuid::Uuid,
    pub objective: i64,
    pub topic: i64
}

impl ObjectiveSatisfier {
    pub async fn commit(&self, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::insert_into(objective_satisfiers::table)
            .values(self)
            .execute(conn)
            .await?;

        Ok(())
    }
}