use rocket_db_pools::diesel::{QueryResult, prelude::*};
use rocket_db_pools::Connection;
use crate::error::{DeductError, DeductResult};
use crate::schema::*;
use crate::model::*;
use std::future::*;

fn flatten_3<A, B, C, E>(tuple: (Result<A, E>, Result<B, E>, Result<C, E>)) -> Result<(A, B, C), E> {
    Ok((tuple.0?, tuple.1?, tuple.2?))
}

#[derive(Debug, Serialize, Deserialize, Associations, Queryable, Insertable, Identifiable)]
#[diesel(table_name = knowledge_graphs, belongs_to(User, foreign_key = author))]
pub struct KnowledgeGraph {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub author: i64,
    pub last_modified: std::time::SystemTime
}

/// A full response to the user that provides all information necessary to render a graph and institute all
/// constraints, such as edges within the graph to structure progress, and requirements outside the graph
/// to indicate a necessary piece of prior knowledge.
#[derive(Serialize)]
pub struct ResponseGraph {
    #[serde(flatten)]
    pub graph: KnowledgeGraph,
    pub topics: Vec<Topic>,
    pub requirements: Vec<(i64, i64)>,
    pub objectives: Vec<(i64, Objective)>
}

impl KnowledgeGraph {
    pub async fn create(user_id: i64, name: String, description: String, conn: &mut Connection<Db>) -> DeductResult<KnowledgeGraph> {
        Ok(diesel::insert_into(knowledge_graphs::table)
            .values((knowledge_graphs::author.eq(user_id), knowledge_graphs::name.eq(name), knowledge_graphs::description.eq(description)))
            .get_result::<KnowledgeGraph>(conn)
            .await?)
    }

    pub async fn get(id: uuid::Uuid, conn: &mut Connection<Db>) -> DeductResult<KnowledgeGraph> {
        Ok(knowledge_graphs::table
            .filter(knowledge_graphs::id.eq(id))
            .first::<KnowledgeGraph>(conn)
            .await?)
    }

    pub async fn get_from_path(username: String, title: String, mut conn: Connection<Db>) -> DeductResult<KnowledgeGraph> {
        let user = User::get_from_username(username, &mut conn).await?;

        Ok(knowledge_graphs::table
            .filter(
                knowledge_graphs::author.eq(user.id)
                .and(knowledge_graphs::name.eq(title))
                )
            .first::<KnowledgeGraph>(&mut conn)
            .await?)
    }

    #[inline(always)]
    pub fn check_owner(&self, id: i64) -> DeductResult<()> {
        if self.author == id {
            Ok(())
        } else {
            Err(DeductError::UnauthorizedUser("User is not authorized to access this graph".to_string()))
        }
    }

    pub async fn delete(self, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(knowledge_graphs::table.filter(knowledge_graphs::id.eq(self.id)))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn delete_topic(&self, topic_id: i64, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(
            topics::table.filter(
                topics::id.eq(topic_id)
                .and(topics::knowledge_graph_id.eq(self.id)))
            )
            .execute(conn)
            .await?;
    
        Ok(())
    }

    pub async fn delete_requirement(&self, requirement_id: i64, conn: &mut Connection<Db>) -> DeductResult<()> {
        diesel::delete(
            requirements::table.filter(
                requirements::id.eq(requirement_id)
                .and(requirements::knowledge_graph_id.eq(self.id)))
            )
            .execute(conn)
            .await?;
    
        Ok(())
    }
    
    pub async fn to_response(self, conn: &mut Connection<Db>) -> DeductResult<ResponseGraph> {
        let topics_query = Topic::belonging_to(&self)
            .load::<Topic>(conn);

        let requirements_query = Requirement::belonging_to(&self)
            .load::<Requirement>(conn);
            //.iter()
            //.map(|x| (x.source, x.destination))
            //.collect();

        let obj_pre_query = objective_prerequisites::table
            .inner_join(objectives::table)
            .filter(
                objective_prerequisites::knowledge_graph_id.eq(self.id)
                .and(objective_prerequisites::topic_to_objective.eq(false))
            )
            .load::<(ObjectivePrerequisite, Objective)>(conn);

        //let (topics, requirements, objectives) = flatten_3(std::future::join!(topics_query, requirements_query, obj_pre_query).await)?;

        // The goal here is to wait until std::future::join! is stablized
        let topics = topics_query.await?;
        let requirements = requirements_query.await?;
        let objectives = obj_pre_query.await?;
    
        Ok(ResponseGraph {
            graph: self,

            topics: topics,

            requirements: requirements
                .iter()
                .map(|req| (req.source, req.destination))
                .collect(),

            objectives: objectives
                .iter()
                .cloned()
                .map(|(prereq, obj)| (prereq.topic, obj))
                .collect()
        })
    }

}


/// Represents an incoming request to create a `KnowledgeGraph`.
#[derive(Deserialize)]
pub struct KnowledgeGraphCreation {
    pub name: String,
    pub description: String
}

/// Represents a result in a search for graphs. Provides the ID of the graph (to
/// access the link), as well as the title, description, and author of the graph.
#[derive(Serialize)]
pub struct SearchResultGraph {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub author: String
}