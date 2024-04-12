use rocket_db_pools::diesel::{QueryResult, prelude::*};
use rocket_db_pools::Connection;
use crate::error::{DeductError, DeductResult};
use crate::schema::*;
use crate::model::{Db, KnowledgeGraph, Objective};
use rocket_db_pools::diesel::query_dsl::LoadQuery;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Clone, Identifiable, Associations)]
#[diesel(table_name = objective_prerequisites, belongs_to(KnowledgeGraph), primary_key(knowledge_graph_id, topic, objective))]
pub struct ObjectivePrerequisite {
    pub knowledge_graph_id: uuid::Uuid,
    pub topic: i64,
    pub objective: i64,
    pub topic_to_objective: bool
}
