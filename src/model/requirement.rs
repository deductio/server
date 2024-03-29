use rocket_db_pools::diesel::{QueryResult, prelude::*};
use serde::{Deserialize, Serialize};
use crate::schema::*;
use crate::model::KnowledgeGraph;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(KnowledgeGraph), table_name = requirements)]

pub struct Requirement {
    pub source: i64,
    pub destination: i64,
    pub knowledge_graph_id: uuid::Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>
}