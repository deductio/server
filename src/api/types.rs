use crate::model::knowledge_graph::*;
use crate::model::topic::*;
use crate::model::Db;
use crate::model::Requirement;
use rocket::serde::Serialize;
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::{QueryResult, prelude::*};

#[derive(Serialize)]
pub struct ResponseGraph {
    #[serde(flatten)]
    pub graph: KnowledgeGraph,
    pub topics: Vec<Topic>,
    pub requirements: Vec<(i64, i64)>
}

use crate::schema::knowledge_graphs as KGTable;

// absolutely need to change this format
impl ResponseGraph {
    pub async fn get_graph(graph_id: uuid::Uuid, mut conn: Connection<Db>) -> QueryResult<ResponseGraph> {
        use KGTable::dsl::*;

        let graph = KGTable::table
            .filter(id.eq(graph_id))
            .first::<KnowledgeGraph>(&mut conn).await?;

        let topics = Topic::belonging_to(&graph)
            .load::<Topic>(&mut conn).await?;

        let requirements = Requirement::belonging_to(&graph)
            .load::<Requirement>(&mut conn).await?
            .iter()
            .map(|x| (x.source, x.destination))
            .collect();

        Ok(ResponseGraph {
            graph: graph,
            topics: topics,
            requirements: requirements
        })
    }
}

/// Represents an incoming request to create a `KnowledgeGraph`.
#[derive(Deserialize)]
pub struct KnowledgeGraphCreation {
    pub name: String,
    pub description: String
}