use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};
use crate::error::DeductResult;
use crate::schema::*;
use crate::model::{Db, KnowledgeGraph, Requirement};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable, QueryableByName)]
#[diesel(belongs_to(KnowledgeGraph), table_name = topics)]

pub struct Topic {
    pub knowledge_graph_id: uuid::Uuid,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    pub subject: String,
    pub content: serde_json::Value
}

impl Topic {
    pub async fn commit(&self, conn: &mut Connection<Db>) -> DeductResult<Topic> {
        Ok(diesel::insert_into(topics::table)
            .values(self)
            .on_conflict(topics::id)
            .do_update()
            .set((topics::content.eq(self.content.clone()), topics::title.eq(self.title.clone()), topics::subject.eq(self.subject.clone())))
            .get_result(conn)
            .await?)
    }

    pub fn requires(&self, other: &Topic) -> Option<Requirement> {
        let source_id = self.id?;
        let destination_id = other.id?;

        Some(Requirement { id: None, source: source_id, destination: destination_id, knowledge_graph_id: self.knowledge_graph_id })
    }
}
