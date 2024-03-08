use rocket_db_pools::Connection;
use rocket::serde::json::Json;
use crate::model::Db;
use rocket_db_pools::diesel::{QueryResult, prelude::*};

use crate::api::types::ResponseGraph;
use crate::model::KnowledgeGraph;
use crate::model::Topic;

use crate::schema::knowledge_graphs as KGTable;

#[get("/<graph_id>")]
pub async fn get_graph(graph_id: uuid::Uuid, mut conn: Connection<Db>) -> QueryResult<Json<ResponseGraph>> {
    use KGTable::dsl::*;

    let graph = KGTable::table
        .filter(id.eq(graph_id))
        .first::<KnowledgeGraph>(&mut conn).await?;

    let topics = Topic::belonging_to(&graph)
        .load::<Topic>(&mut conn).await?;

    Ok(Json(ResponseGraph { graph: graph, topics: topics }))
}