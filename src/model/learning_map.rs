use std::collections::HashSet;
use crate::api::error::DeductResult;
use crate::users::AuthenticatedUser;
use diesel::sql_types::BigInt;
use rocket_db_pools::diesel::dsl::sql_query;
use rocket_db_pools::diesel::sql_types::{Uuid, Nullable};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use crate::model::{Db, User, Topic};
use crate::schema::*;
 
#[derive(Queryable, Insertable, Identifiable, Associations)]
#[diesel(table_name = learning_maps, belongs_to(User, foreign_key = user_id))]
pub struct LearningMap {
    pub id: i64,
    pub user_id: i64,
    pub title: String
}

#[derive(Serialize, Selectable, Queryable)]
#[diesel(table_name = learning_maps)]
pub struct SimpleLearningMap {
    id: i64,
    title: String
}

#[derive(Queryable)]
#[diesel(table_name = learning_map_goals, belongs_to(LearningMap))]
pub struct LearningMapGoal {
    pub learning_map_id: i64,
    pub topic_id: i64
}

#[derive(Serialize)]
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

    #[diesel(sql_type = Nullable<BigInt>)]
    dest_topic: Option<i64>,

    #[diesel(sql_type = Nullable<Uuid>)]
    dest_graph: Option<uuid::Uuid>
}

impl LearningMap {
    pub async fn to_response(self, conn: &mut Connection<Db>) -> DeductResult<ResponseLearningMap> {

        let edges = sql_query("WITH RECURSIVE learning_map_graph(dest_topic, dest_graph, source_topic, source_graph) AS (
            SELECT NULL::bigint, NULL::uuid, topic_id, topics.knowledge_graph_id 
                FROM learning_map_goals 
                INNER JOIN topics ON topic_id = topics.id
                WHERE learning_map_id = $1
        UNION ALL
            SELECT r.destination, r.kg1, r.source, r.kg2 FROM (
        SELECT destination, knowledge_graph_id AS kg1, source, knowledge_graph_id AS kg2
            FROM requirements 
        UNION 
        SELECT topic, knowledge_graph_id, suggested_topic, suggested_graph
            FROM objective_prerequisites op
            WHERE NOT EXISTS (SELECT 1 FROM user_objective_progress WHERE objective_id = op.objective AND user_id = $2)
        ) r, learning_map_graph lmg
                    WHERE r.destination = lmg.source_topic
        ) SELECT * FROM learning_map_graph;")
            .bind::<BigInt, _>(self.id)
            .bind::<BigInt, _>(self.user_id)
            .load::<LearningMapLink>(conn).await?;

        let mut node_ids = HashSet::new();
        let mut graph_ids = HashSet::new();
        let mut requirements = Vec::new();

        for edge in edges {
            if let Some(dest_topic) = edge.dest_topic {
                node_ids.insert(dest_topic);

                requirements.push((edge.source_topic, dest_topic));
            }
            node_ids.insert(edge.source_topic);

            if let Some(dest_graph) = edge.dest_graph {
                graph_ids.insert(dest_graph);
            }

            graph_ids.insert(edge.source_graph);
        }

        let progress = progress::table
            .filter(progress::user_id.eq(self.user_id).and(progress::knowledge_graph_id.eq_any(graph_ids)))
            .select(progress::topic)
            .load::<i64>(conn)
            .await?;

        let topics = topics::table
            .filter(topics::id.eq_any(node_ids))
            .load::<Topic>(conn)
            .await?;

        Ok(ResponseLearningMap {
            requirements: requirements,
            topics: topics,
            progress: progress
        })

    }

    pub async fn add_topic(id: i64, topic: i64, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::insert_into(learning_map_goals::table)
            .values((learning_map_goals::learning_map_id.eq(id), learning_map_goals::topic_id.eq(topic)))
            .execute(conn).await?;

        Ok(())
    }

    pub async fn generate(user: AuthenticatedUser, title: String, topic: i64, conn: &mut Connection<Db>) -> DeductResult<ResponseLearningMap> {
        let insertion: LearningMap = diesel::insert_into(learning_maps::table)
            .values((learning_maps::title.eq(title), learning_maps::user_id.eq(user.db_id)))
            .get_result(conn).await?;

        Self::add_topic(insertion.id, topic, conn).await?;

        insertion.to_response(conn).await 
    }

    pub async fn get_learning_maps(user: AuthenticatedUser, page: i64, conn: &mut Connection<Db>) -> DeductResult<Vec<SimpleLearningMap>> {
        Ok(learning_maps::table
            .filter(learning_maps::user_id.eq(user.db_id))
            .select(SimpleLearningMap::as_select())
            .limit(10)
            .offset(page * 10)
            .load::<SimpleLearningMap>(conn)
            .await?)
    }

    pub async fn get(id: i64, conn: &mut Connection<Db>) -> DeductResult<LearningMap> {
        Ok(learning_maps::table
            .filter(learning_maps::id.eq(id))
            .get_result(conn)
            .await?)
    }
}