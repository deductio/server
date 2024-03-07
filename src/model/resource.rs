use rocket_db_pools::diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::model::Topic;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(Topic), table_name = resources)]
pub struct Resource {
    title: String,
    description: String,
    topic_id: i64,
    link: Option<String>,
    id: i64
}
