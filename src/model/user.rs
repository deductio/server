use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use crate::{error::DeductResult, model::Db};
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

impl User {
    pub async fn get_from_username(username: String, conn: &mut Connection<Db>) -> DeductResult<User> {
        Ok(users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
            .await?)
    }
}