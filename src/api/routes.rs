use rocket_db_pools::Connection;
use rocket::serde::json::Json;
use crate::model::Db;
use crate::types::KnowledgeGraphCreation;
use rocket_db_pools::diesel::{QueryResult, prelude::*};
use crate::model::{KnowledgeGraph, Topic, Requirement};
use crate::error::DeductResult;

use crate::api::types::ResponseGraph;
use crate::api::oauth::{check_user_is_owner, AuthenticatedUser};

#[get("/<graph_id>")]
pub async fn get_graph(graph_id: uuid::Uuid, conn: Connection<Db>) -> QueryResult<Json<ResponseGraph>> {
    let graph = ResponseGraph::get_graph(graph_id, conn).await?;

    Ok(Json(graph))
}

#[post("/create", data = "<data>", format = "json")]
pub async fn create_graph(user: AuthenticatedUser, data: Json<KnowledgeGraphCreation>, mut conn: Connection<Db>) 
    -> DeductResult<Json<KnowledgeGraph>> 
{
    use crate::schema::knowledge_graphs::dsl::*;

    Ok(Json(diesel::insert_into(knowledge_graphs)
        .values((author.eq(user.db_id), name.eq(data.name.clone()), description.eq(data.description.clone())))
        .get_result(&mut conn)
        .await?))
}

#[put("/<graph_id>", data = "<topic>", format = "json")]
pub async fn add_topic(user: AuthenticatedUser, graph_id: uuid::Uuid, topic: Json<Topic>,
    mut conn: Connection<Db>) 
    -> DeductResult<Json<Topic>>
{
    use crate::schema::topics::dsl::*;

    check_user_is_owner(graph_id, &mut conn, user).await?;

    Ok(Json(diesel::insert_into(topics)
        .values(&*topic)
        .on_conflict(id)
        .do_update()
        .set((content.eq(topic.content.clone()), title.eq(topic.title.clone()), subject.eq(topic.subject.clone())))
        .get_result(&mut conn)
        .await?))

}

#[put("/<graph_id>", data = "<requirement>", format = "json")]
pub async fn add_requirement(user: AuthenticatedUser, graph_id: uuid::Uuid, requirement: Json<Requirement>, 
    mut conn: Connection<Db>) 
    -> DeductResult<Json<Requirement>>
{
    use crate::schema::requirements::dsl::*;

    check_user_is_owner(graph_id, &mut conn, user).await?;

    Ok(Json(diesel::insert_into(requirements)
        .values(&*requirement)
        .on_conflict(id)
        .do_update()
        .set((source.eq(requirement.source), destination.eq(requirement.destination)))
        .get_result(&mut conn)
        .await?))
}

#[delete("/<graph_id>?<topic>")]
pub async fn delete_topic(user: AuthenticatedUser, graph_id: uuid::Uuid, topic: i64, mut conn: Connection<Db>) 
    -> DeductResult<()> 
{
    check_user_is_owner(graph_id, &mut conn, user).await?;

    use crate::schema::topics::dsl::*;

    diesel::delete(
        topics.filter(
            id.eq(topic)
            .and(knowledge_graph_id.eq(graph_id)))
        )
        .execute(&mut conn)
        .await?;

    Ok(())
}

#[delete("/<graph_id>?<requirement>")]
pub async fn delete_requirement(user: AuthenticatedUser, graph_id: uuid::Uuid, requirement: i64, 
    mut conn: Connection<Db>)
    -> DeductResult<()>
{
    check_user_is_owner(graph_id, &mut conn, user).await?;

    use crate::schema::requirements::dsl::*;

    diesel::delete(
        requirements.filter(
            id.eq(requirement)
            .and(knowledge_graph_id.eq(graph_id)))
        )
        .execute(&mut conn)
        .await?;

    Ok(())
}

#[delete("/<graph_id>")]
pub async fn delete_graph(user: AuthenticatedUser, graph_id: uuid::Uuid, mut conn: Connection<Db>) -> DeductResult<()> {
    check_user_is_owner(graph_id, &mut conn, user).await?;

    use crate::schema::knowledge_graphs::dsl::*;

    diesel::delete(knowledge_graphs.filter(id.eq(graph_id)))
        .execute(&mut conn)
        .await?;

    Ok(())
}
