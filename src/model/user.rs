use rocket_db_pools::diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub github_user_id: Option<String>,
    pub google_user_id: Option<String>,
    pub username: String,
    pub avatar: Option<String>,
    pub id: i64
}