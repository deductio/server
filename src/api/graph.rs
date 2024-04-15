use rocket::form::Form;
use rocket_db_pools::Connection;
use rocket::serde::json::Json;
use crate::model::{Db, KnowledgeGraph, Topic, Requirement};
use crate::model::knowledge_graph::{KnowledgeGraphCreation, ResponseGraph};
use crate::error::DeductResult;
use crate::api::search::SearchResultGraph;
use crate::api::users::AuthenticatedUser;

#[get("/view/<graph_id>")]
pub async fn get_graph(graph_id: uuid::Uuid, mut conn: Connection<Db>) -> DeductResult<Json<ResponseGraph>> {
    let graph: KnowledgeGraph = KnowledgeGraph::get(graph_id, &mut conn).await?;

     let response: ResponseGraph = graph.to_response(&mut conn).await?;

    Ok(Json(response))
}

#[get("/view/<username>/<title>")]
pub async fn get_graph_from_username(username: String, title: String, mut conn: Connection<Db>) -> DeductResult<Json<ResponseGraph>> {
    let graph = KnowledgeGraph::get_from_path(username, title, &mut conn).await?;

    let response: ResponseGraph = graph.to_response(&mut conn).await?;

    Ok(Json(response))
}

#[post("/create", data = "<data>")]
pub async fn create_graph(user: AuthenticatedUser, data: Form<KnowledgeGraphCreation>, mut conn: Connection<Db>) 
    -> DeductResult<Json<KnowledgeGraph>> 
{
    Ok(Json(KnowledgeGraph::create(user.db_id, data.name.clone(), data.description.clone(), &mut conn).await?))
    
}

#[put("/edit/<graph_id>/topic", data = "<topic>", format = "json")]
pub async fn add_topic(user: AuthenticatedUser, graph_id: uuid::Uuid, topic: Json<Topic>,
    mut conn: Connection<Db>) 
    -> DeductResult<Json<Topic>>
{
    let graph = KnowledgeGraph::get(graph_id, &mut conn).await?;
    graph.check_owner(user.db_id)?;

    Ok(Json((*topic).commit(&mut conn).await?))

}

#[put("/edit/<graph_id>/requirement", data = "<requirement>", format = "json")]
pub async fn add_requirement(user: AuthenticatedUser, graph_id: uuid::Uuid, requirement: Json<Requirement>, 
    mut conn: Connection<Db>) 
    -> DeductResult<Json<Requirement>>
{
    let graph: KnowledgeGraph = KnowledgeGraph::get(graph_id, &mut conn).await?;

    graph.check_owner(user.db_id)?;

    Ok(Json((*requirement).commit(&mut conn).await?))
}

#[delete("/edit/<graph_id>/topic?<topic>")]
pub async fn delete_topic(user: AuthenticatedUser, graph_id: uuid::Uuid, topic: i64, mut conn: Connection<Db>) 
    -> DeductResult<()> 
{
    let graph: KnowledgeGraph = KnowledgeGraph::get(graph_id, &mut conn).await?;

    graph.check_owner(user.db_id)?;
    Ok(graph.delete_topic(topic, &mut conn).await?)
    
}

#[delete("/edit/<graph_id>/requirement?<requirement>")]
pub async fn delete_requirement(user: AuthenticatedUser, graph_id: uuid::Uuid, requirement: i64, 
    mut conn: Connection<Db>)
    -> DeductResult<()>
{
    let graph: KnowledgeGraph = KnowledgeGraph::get(graph_id, &mut conn).await?;

    graph.check_owner(user.db_id)?;
    Ok(graph.delete_requirement(requirement, &mut conn).await?)
}

#[delete("/edit/<graph_id>")]
pub async fn delete_graph(user: AuthenticatedUser, graph_id: uuid::Uuid, mut conn: Connection<Db>) -> DeductResult<()> {
    let graph: KnowledgeGraph = KnowledgeGraph::get(graph_id, &mut conn).await?;

    graph.check_owner(user.db_id)?;
    Ok(graph.delete(&mut conn).await?)
}

