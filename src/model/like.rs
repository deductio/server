use crate::schema::*;
use crate::api::error::DeductResult;
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use crate::model::Db;

#[derive(Queryable, Insertable)]
#[diesel(table_name = likes)]
pub struct Like {
    pub knowledge_graph_id: uuid::Uuid,
    pub user_id: i64,
    pub like_date: chrono::NaiveDate
}

impl Like {
    pub async fn insert(graph_id: uuid::Uuid, user_id: i64, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::insert_into(likes::table)
            .values((likes::knowledge_graph_id.eq(graph_id), likes::user_id.eq(user_id)))
            .on_conflict((likes::knowledge_graph_id, likes::user_id))
            .do_nothing()
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn delete(graph_id: uuid::Uuid, user_id: i64, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(likes::table
            .filter(likes::knowledge_graph_id.eq(graph_id).and(likes::user_id.eq(user_id))))
            .execute(conn)
            .await?;

        Ok(())
    }
}