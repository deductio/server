use rocket_db_pools::Connection;
use rocket::serde::json::Json;
use crate::model::Db;
use rocket_db_pools::diesel::{QueryResult, prelude::*};

use crate::api::types::ResponseGraph;

#[get("/<graph_id>")]
pub async fn get_graph(graph_id: uuid::Uuid, conn: Connection<Db>) -> QueryResult<Json<ResponseGraph>> {
    let graph = ResponseGraph::get_graph(graph_id, conn).await?;

    Ok(Json(graph))
}

/* 
#[post("/create")]
pub async fn create_graph()
*/