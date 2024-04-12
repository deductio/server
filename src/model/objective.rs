use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use crate::error::DeductResult;
use crate::schema::*;
use crate::model::Db;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[diesel(table_name = objectives)]
pub struct Objective {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    pub title: String,
    pub description: String
}

impl Objective {
    pub async fn get(id: i64, mut conn: Connection<Db>) -> DeductResult<Objective> {
        Ok(objectives::table.filter(objectives::id.eq(id)).first(&mut conn).await?)
    }
}