use rocket_db_pools::diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;
use crate::model::KnowledgeGraph;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(KnowledgeGraph), table_name = topics)]
pub struct Topic {
    pub knowledge_graph_id: uuid::Uuid,
    pub knowledge_graph_index: i32,
    pub title: String,
    pub id: i64,
    pub subject: String,
    pub content: String
}