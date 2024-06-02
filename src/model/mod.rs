use rocket_db_pools::Database;
use rocket_db_pools::diesel::PgPool;


#[derive(Database)]
#[database("deductio")]
pub struct Db(pub PgPool);

pub mod knowledge_graph;
pub mod topic;
pub mod requirement;
pub mod user;
pub mod objective;
pub mod progress;
pub mod like;
pub mod learning_map;

pub use crate::model::knowledge_graph::KnowledgeGraph;
pub use crate::model::topic::Topic;
pub use crate::model::requirement::Requirement;
pub use crate::model::user::User;
pub use crate::model::objective::Objective;
pub use crate::model::progress::Progress;
pub use crate::model::like::Like;
pub use crate::model::learning_map::LearningMap;