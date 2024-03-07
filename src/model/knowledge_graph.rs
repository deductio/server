use rocket_db_pools::diesel::prelude::*;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = knowledge_graphs)]
pub struct KnowledgeGraph {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub owner: String,
}