use std::collections::{HashMap, HashSet};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use crate::error::DeductResult;
use crate::schema::*;
use crate::model::*;
use crate::model::topic::PreviewTopic;
use crate::search::SearchResultGraph;
use crate::users::AuthenticatedUser;
use diesel_full_text_search::TsVectorExtensions;
use crate::model::knowledge_graph::PreviewGraph;

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable, Clone)]
#[diesel(table_name = objectives)]
pub struct Objective {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    pub title: String,
    pub description: String
}

diesel::joinable!(objective_satisfiers -> topics (topic));

#[derive(Serialize)]
pub struct ObjectiveSatisfierSearchResult {
    pub graph: SearchResultGraph,
    pub topics: Vec<PreviewTopic>
}

impl Objective {
    pub async fn get(id: i64, mut conn: Connection<Db>) -> DeductResult<Objective> {
        Ok(objectives::table.filter(objectives::id.eq(id)).select(Objective::as_select()).first(&mut conn).await?)
    }

    pub async fn create(user: AuthenticatedUser, title: String, description: String, conn: &mut Connection<Db>) -> DeductResult<Objective> {
        let res = diesel::insert_into(objectives::table)
            .values((
                objectives::title.eq(title), 
                objectives::description.eq(description),
                objectives::author.eq(user.db_id)
            ))
            .returning(Objective::as_select())
            .get_result(conn).await?;

        Ok(res)
    }

    pub async fn get_satisfied_graphs(id: i64, conn: &mut Connection<Db>) -> DeductResult<Vec<ObjectiveSatisfierSearchResult>> {
        let res: Vec<(PreviewTopic, KnowledgeGraph, User)> = objective_satisfiers::table
            .filter(objective_satisfiers::objective.eq(id))
            .inner_join(
                topics::table
                    .inner_join(knowledge_graphs::table
                        .inner_join(users::table)
                    )
            )
            .select((PreviewTopic::as_select(), KnowledgeGraph::as_select(), User::as_select()))
            .load::<(PreviewTopic, KnowledgeGraph, User)>(conn)
            .await?;

        let mut map: HashMap<uuid::Uuid, Vec<PreviewTopic>> = HashMap::new();

        for (topic, graph, _) in &res {
            if let Some(vec) = map.get_mut(&graph.id) {
                vec.push(topic.clone());
            } else {
                map.insert(graph.id, vec![topic.clone()]);
            }
        }

        let search_vec: HashSet<(KnowledgeGraph, User)> = HashSet::from_iter(res.into_iter().map(|(_, graph, user)| (graph, user)));

        let search_res = SearchResultGraph::get_likes(search_vec.into_iter().collect(), None, conn).await?;

        Ok(search_res.into_iter()
            .map(|search_graph| ObjectiveSatisfierSearchResult {
                topics: map.remove(&search_graph.id).unwrap(), // SAFETY: guaranteed to not panic with above logic
                graph: search_graph
            })
            .collect()
        )

    }

    pub async fn search_objectives(query: String, page: i64, conn: &mut Connection<Db>) -> DeductResult<Vec<Objective>> {
        Ok(objectives::table
            .filter(objectives::tsv_title_desc.matches(diesel_full_text_search::websearch_to_tsquery(query)))
            .limit(10)
            .offset(page * 10)
            .select(Objective::as_select())
            .load::<Objective>(conn)
            .await?)
    }
}

#[derive(Debug, Deserialize, Queryable, Insertable, Clone, Identifiable, Associations, Selectable)]
#[diesel(table_name = objective_prerequisites, belongs_to(KnowledgeGraph), primary_key(knowledge_graph_id, topic, objective))]
pub struct ObjectivePrerequisite {
    pub knowledge_graph_id: uuid::Uuid,
    pub topic: i64,
    pub objective: i64,
    pub suggested_topic: i64,
    pub suggested_graph: uuid::Uuid
}

impl ObjectivePrerequisite {
    pub async fn commit(&self, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::insert_into(objective_prerequisites::table)
            .values(self)
            .execute(conn)
            .await?;

        Ok(())
    }
}

#[derive(Serialize)]
pub struct ResponseObjPrerequisite {
    pub knowledge_graph_id: uuid::Uuid,
    pub topic: i64,
    pub objective: Objective,
    pub suggested_topic: i64,
    pub suggested_graph: PreviewGraph,
    pub satisfied: bool
}

#[derive(Insertable, Queryable, Deserialize, Serialize, Selectable)]
pub struct ObjectiveSatisfier {
    pub knowledge_graph_id: uuid::Uuid,
    pub objective: i64,
    pub topic: i64
}

#[derive(Serialize)]
pub struct ResponseObjSatisfier {
    pub topic: i64,
    pub objective: Objective
}

impl ObjectiveSatisfier {
    pub async fn commit(&self, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::insert_into(objective_satisfiers::table)
            .values(self)
            .execute(conn)
            .await?;

        Ok(())
    }
}