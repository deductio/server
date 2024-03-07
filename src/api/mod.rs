use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{QueryResult, prelude::*};
use crate::model::Db;
use crate::model::knowledge_graph::*;
use crate::model::topic::*;
use crate::model::resource::*;

pub mod routes;
pub mod types;