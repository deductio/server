use crate::model::knowledge_graph::*;
use crate::model::topic::*;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ResponseGraph {
    #[serde(flatten)]
    pub graph: KnowledgeGraph,

    pub topics: Vec<Topic>
}