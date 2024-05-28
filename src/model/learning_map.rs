use crate::{model::knowledge_graph::ResponseGraph, schema::learning_map_requirements};
use crate::api::error::DeductResult;
use crate::schema::learning_maps;
use crate::users::AuthenticatedUser;
use diesel::sql_types::BigInt;
use rocket_db_pools::diesel::dsl::sql_query;
use rocket_db_pools::diesel::sql_types::Uuid;
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use crate::model::{Db, User, Topic};
use crate::schema::*;
 
#[derive(Queryable, Insertable, Identifiable, Associations)]
#[diesel(table_name = learning_maps, belongs_to(User, foreign_key = user_id))]
pub struct LearningMap {
    user_id: i64,
    id: i64,
    title: String
}

#[derive(Queryable)]
#[diesel(table_name = learning_map_goals, belongs_to(LearningMap))]
pub struct LearningMapGoal {
    pub learning_map_id: i64,
    pub topic_id: i64
}

pub struct ResponseLearningMap {
    requirements: Vec<(i64, i64)>,
    topics: Vec<Topic>,
    progress: Vec<i64>
}

#[derive(QueryableByName)]
pub struct LearningMapLink {
    #[diesel(sql_type = BigInt)]
    source_topic: i64,

    #[diesel(sql_type = Uuid)]
    source_graph: uuid::Uuid,

    #[diesel(sql_type = BigInt)]
    dest_topic: i64,

    #[diesel(sql_type = Uuid)]
    dest_graph: uuid::Uuid
}

impl LearningMap {
    pub async fn to_response(id: i64, conn: &mut Connection<Db>) -> DeductResult<ResponseLearningMap> {

        let goals = learning_map_goals::table
            .filter(learning_map_goals::learning_map_id.eq(id))
            .inner_join(topics::table)
            .load::<(LearningMapGoal, Topic)>(conn).await?;

        let edges = sql_query("WITH RECURSIVE learning_map_graph(dest_topic, dest_graph, source_topic, source_graph) AS (
            VALUES (NULL::bigint, NULL::uuid, ?, ?)
        UNION ALL
            SELECT r.destination, r.kg1, r.source, r.kg2 FROM (
        SELECT destination, knowledge_graph_id AS kg1, source, knowledge_graph_id AS kg2
            FROM requirements 
        UNION 
        SELECT topic, knowledge_graph_id, suggested_topic, suggested_graph
            FROM objective_prerequisites op
            WHERE NOT EXISTS (SELECT 1 FROM user_objective_progress WHERE objective_id = op.objective AND user_id = ?)
        ) r, learning_map_graph lmg
                    WHERE r.destination = lmg.source_topic
        ) SELECT * FROM learning_map_graph WHERE dest_graph <> source_graph;")
            .bind::<BigInt, _>(topic)
            .bind::<BigInt, _>(insertion.id)
            .bind::<BigInt, _>(user.db_id)
            .load::<LearningMapLink>(conn).await?;
    }

    pub async fn generate(user: AuthenticatedUser, title: String, topic: i64, conn: &mut Connection<Db>) -> DeductResult<ResponseLearningMap> {
        let insertion: LearningMap = diesel::insert_into(learning_maps::table)
            .values((learning_maps::title.eq(title), learning_maps::user_id.eq(user.db_id)))
            .get_result(conn).await?;

        Self::to_response(insertion.id, conn).await 

    }
}