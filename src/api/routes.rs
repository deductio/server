use rocket_db_pools::Connection;
use rocket::serde::json::Json;
use crate::model::Db;
use rocket_db_pools::diesel::{QueryResult, prelude::*};

use crate::api::types::ResponseGraph;
use crate::model::KnowledgeGraph;
use crate::model::Topic;
use crate::model::Resource;

use crate::schema::knowledge_graphs as KGTable;
use crate::schema::resources as ResTable;

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

#[get("/<_graph_id>/topic/<t_id>")]
pub async fn get_topic_resources(_graph_id: uuid::Uuid, t_id: i64, mut conn: Connection<Db>) 
    -> QueryResult<Json<Vec<Resource>>> {
    
    use ResTable::dsl::*;

    let res = ResTable::table
        .filter(topic_id.eq(t_id))
        .load::<Resource>(&mut conn).await?;

    Ok(Json(res))
}