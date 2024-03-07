use crate::model::knowledge_graph::*;
use crate::model::topic::*;
use crate::model::resource::*;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ResponseGraph {
    #[serde(flatten)]
    pub graph: KnowledgeGraph,

    pub topics: Vec<Topic>
}

#[derive(Serialize)]
pub struct ResponseResource(Resource);
