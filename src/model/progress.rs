use crate::model::Db;
use crate::schema::progress;
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use crate::api::error::DeductResult;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Progress {
    pub user_id: i64,
    pub knowledge_graph_id: uuid::Uuid,
    pub topic: i64
}

impl Progress {
    pub async fn get_user_progress(user_id: i64, knowledge_graph_id: uuid::Uuid, conn: &mut Connection<Db>) -> DeductResult<Vec<Progress>> {
        Ok(progress::table.filter(
            progress::user_id.eq(user_id).and(progress::knowledge_graph_id.eq(knowledge_graph_id)))
            .load::<Progress>(conn)
            .await?
        )
    }

    pub async fn add_progress(user_id: i64, knowledge_graph_id: uuid::Uuid, topic: i64, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::insert_into(progress::table)
            .values((progress::user_id.eq(user_id), progress::knowledge_graph_id.eq(knowledge_graph_id), progress::topic.eq(topic)))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn delete_progress(user_id: i64, knowledge_graph_id: uuid::Uuid, topic: i64, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(
            progress::table.filter(
                progress::user_id.eq(user_id)
                .and(progress::knowledge_graph_id.eq(knowledge_graph_id))
                .and(progress::topic.eq(topic))))
            .execute(conn)
            .await?;

        Ok(())
    }
}

