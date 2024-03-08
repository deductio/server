use rocket_db_pools::Database;
use rocket_db_pools::diesel::PgPool;

#[derive(Database)]
#[database("deductio")]
pub struct Db(PgPool);

pub mod knowledge_graph;
pub mod topic;

pub use crate::model::knowledge_graph::KnowledgeGraph;
pub use crate::model::topic::Topic;