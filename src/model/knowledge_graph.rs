use rocket_db_pools::diesel::{QueryResult, prelude::*};
use crate::schema::*;
use crate::model::User;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = knowledge_graphs, belongs_to(User))]

pub struct KnowledgeGraph {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub author: i64
}
