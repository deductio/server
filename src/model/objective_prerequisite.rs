use rocket_db_pools::diesel::prelude::*;
use crate::schema::*;
use crate::model::KnowledgeGraph;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Clone, Identifiable, Associations, Selectable)]
#[diesel(table_name = objective_prerequisites, belongs_to(KnowledgeGraph), primary_key(knowledge_graph_id, topic, objective))]
pub struct ObjectivePrerequisite {
    pub knowledge_graph_id: uuid::Uuid,
    pub topic: i64,
    pub objective: i64
}
