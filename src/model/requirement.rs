use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};
use crate::error::DeductResult;
use crate::schema::*;
use crate::model::Db;
use crate::model::KnowledgeGraph;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(KnowledgeGraph), table_name = requirements)]

pub struct Requirement {
    pub source: i64,
    pub destination: i64,
    pub knowledge_graph_id: uuid::Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>
}

impl Requirement {
    pub async fn commit(&self, conn: &mut Connection<Db>) -> DeductResult<Requirement> {
        Ok(diesel::insert_into(requirements::table)
            .values((requirements::knowledge_graph_id.eq(self.knowledge_graph_id), requirements::destination.eq(self.destination), 
                requirements::source.eq(self.source)))
            .on_conflict(requirements::id)
            .do_update()
            .set((requirements::source.eq(self.source), requirements::destination.eq(self.destination)))
            .get_result(conn)
            .await?)
    }
}