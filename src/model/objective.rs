use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use crate::error::DeductResult;
use crate::schema::*;
use crate::model::{Db, KnowledgeGraph, User};
use crate::search::SearchResultGraph;
use crate::users::AuthenticatedUser;

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
}