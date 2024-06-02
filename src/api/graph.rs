pub mod view {
    use rocket_db_pools::Connection;
    use rocket::serde::json::Json;
    use crate::model::*;
    use crate::model::knowledge_graph::ResponseGraph;
    use crate::error::DeductResult;
    use crate::api::users::AuthenticatedUser;

    #[get("/<graph_id>")]
    pub async fn get_graph(graph_id: uuid::Uuid, user: Option<AuthenticatedUser>, mut conn: Connection<Db>) -> DeductResult<Json<ResponseGraph>> {
        let graph: KnowledgeGraph = KnowledgeGraph::get(graph_id, &mut conn).await?;
    
        let response: ResponseGraph = graph.to_response(user, &mut conn).await?;
    
        Ok(Json(response))
    }

    #[get("/<username>/<title>")]
    pub async fn get_graph_from_username(username: String, title: String, user: Option<AuthenticatedUser>, mut conn: Connection<Db>) -> DeductResult<Json<ResponseGraph>> {
        let graph = KnowledgeGraph::get_from_path(username, title, &mut conn).await?;

        let response: ResponseGraph = graph.to_response(user, &mut conn).await?;

        Ok(Json(response))
    }
}

pub mod progress {
    use rocket_db_pools::Connection;
    use crate::model::{Progress, Db};
    use crate::error::DeductResult;
    use crate::api::users::AuthenticatedUser;
    
    #[put("/<graph_id>?<topic>")]
    pub async fn put_progress(graph_id: uuid::Uuid, user: AuthenticatedUser, topic: i64, mut conn: Connection<Db>) -> DeductResult<()> {
        Progress::add_progress(user.db_id, graph_id, topic, &mut conn).await
    }
    
    #[delete("/<graph_id>?<topic>")]
    pub async fn delete_progress(graph_id: uuid::Uuid, user: AuthenticatedUser, topic: i64, mut conn: Connection<Db>) -> DeductResult<()> {
        Progress::delete_progress(user.db_id, graph_id, topic, &mut conn).await
    }
}

pub mod create {
    use rocket::form::Form;
    use rocket_db_pools::Connection;
    use rocket::serde::json::Json;
    use crate::model::*;
    use crate::model::knowledge_graph::KnowledgeGraphCreation;
    use crate::error::DeductResult;
    use crate::api::users::AuthenticatedUser;

    #[post("/", data = "<data>")]
    pub async fn create_graph(user: AuthenticatedUser, data: Form<KnowledgeGraphCreation>, mut conn: Connection<Db>) 
        -> DeductResult<Json<KnowledgeGraph>> 
    {
        Ok(Json(KnowledgeGraph::create(user.db_id, data.name.clone(), data.description.clone(), &mut conn).await?))   
    }
}

pub mod edit {
    use rocket_db_pools::Connection;
    use rocket::form::Form;
    use rocket::serde::json::Json;
    use crate::model::*;
    use crate::error::DeductResult;
    use crate::model::knowledge_graph::{KnowledgeGraphCreation, ResponseGraph};
    use crate::api::users::{AuthenticatedUser, AuthorizedUser};
    use crate::model::objective::{ObjectivePrerequisite, ObjectiveSatisfier};
    use rocket::http::{Cookie, CookieJar, SameSite};

    #[get("/<graph_id>")]
    pub async fn start_editing_graph(user: AuthenticatedUser, cookies: &CookieJar<'_>, graph_id: uuid::Uuid, mut conn: Connection<Db>) -> DeductResult<Json<ResponseGraph>> {
        let graph = KnowledgeGraph::get(graph_id, &mut conn).await?;

        if graph.author != user.db_id {
            return Err(crate::error::DeductError::UnauthorizedUser("User is not authorized to edit this graph"))
        }

        cookies.remove_private("knowledge_graph_id");

        cookies.add_private(Cookie::build(("knowledge_graph_id", graph_id.to_string()))
            .same_site(SameSite::Lax)
            .expires(None)
        );

        Ok(Json(graph.to_response(Some(user), &mut conn).await?))
    }

    #[put("/<graph_id>", data = "<data>")]
    pub async fn modify_graph_info(user: AuthorizedUser, graph_id: uuid::Uuid, data: Form<KnowledgeGraphCreation>, mut conn: Connection<Db>) -> DeductResult<()> {
        user.check_owner(graph_id)?;

        KnowledgeGraph::update_info(graph_id, data.name.clone(), data.description.clone(), &mut conn).await
    }

    #[put("/<graph_id>/topic", data = "<topic>", format = "json")]
    pub async fn add_topic(user: AuthorizedUser, graph_id: uuid::Uuid, topic: Json<Topic>,
        mut conn: Connection<Db>) 
        -> DeductResult<Json<Topic>>
    {
        user.check_owner(graph_id)?;
    
        Ok(Json((*topic).commit(&mut conn).await?))
    
    }
    
    #[put("/<graph_id>/requirement", data = "<requirement>", format = "json")]
    pub async fn add_requirement(user: AuthorizedUser, graph_id: uuid::Uuid, requirement: Json<Requirement>, 
        mut conn: Connection<Db>) 
        -> DeductResult<Json<Requirement>>
    {
        user.check_owner(graph_id)?;
    
        Ok(Json((*requirement).commit(&mut conn).await?))
    }
    
    #[delete("/<graph_id>/topic?<topic>")]
    pub async fn delete_topic(user: AuthorizedUser, graph_id: uuid::Uuid, topic: i64, mut conn: Connection<Db>) 
        -> DeductResult<()> 
    {
        user.check_owner(graph_id)?;

        KnowledgeGraph::delete_topic(graph_id, topic, &mut conn).await
        
    }
    
    #[delete("/<graph_id>/requirement?<src>&<dest>")]
    pub async fn delete_requirement(user: AuthorizedUser, graph_id: uuid::Uuid, src: i64, dest: i64,
        mut conn: Connection<Db>)
        -> DeductResult<()>
    {
        user.check_owner(graph_id)?;

        KnowledgeGraph::delete_requirement(graph_id, (src, dest), &mut conn).await
    }

    #[put("/<graph_id>/objsts", data = "<form>")]
    pub async fn add_objective_satisfier(user: AuthorizedUser, graph_id: uuid::Uuid, form: Form<ObjectiveSatisfier>, mut conn: Connection<Db>) -> DeductResult<()> {    
        if form.knowledge_graph_id != graph_id {
            return Err(crate::error::DeductError::UnauthorizedUser("Reported graph ID's do not match"));
        }

        user.check_owner(graph_id)?;

        (*form).commit(&mut conn).await
    }

    #[delete("/<graph_id>/objsts?<topic>")]
    pub async fn delete_objective_satisfier(user: AuthorizedUser, graph_id: uuid::Uuid, topic: i64, mut conn: Connection<Db>) -> DeductResult<()> {
        user.check_owner(graph_id)?;

        KnowledgeGraph::delete_satisfier(graph_id, topic, &mut conn).await
    }

    #[put("/<graph_id>/objpre", data = "<form>")]
    pub async fn add_objective_prerequisite(user: AuthorizedUser, graph_id: uuid::Uuid, form: Form<ObjectivePrerequisite>, mut conn: Connection<Db>) -> DeductResult<()> {
        if form.knowledge_graph_id != graph_id {
            return Err(crate::error::DeductError::UnauthorizedUser("Reported graph ID's do not match"));
        }

        user.check_owner(graph_id)?;

        (*form).commit(&mut conn).await
    }

    #[delete("/<graph_id>/objpre?<src>&<dest>")]
    pub async fn delete_objective_prerequisite(user: AuthorizedUser, graph_id: uuid::Uuid, src: i64, dest: i64, mut conn: Connection<Db>) -> DeductResult<()> {
        user.check_owner(graph_id)?;

        KnowledgeGraph::delete_prerequisite(graph_id, src, dest, &mut conn).await
    }
    
    #[delete("/<graph_id>")]
    pub async fn delete_graph(user: AuthorizedUser, graph_id: uuid::Uuid, mut conn: Connection<Db>) -> DeductResult<()> {
        user.check_owner(graph_id)?;

        KnowledgeGraph::delete(graph_id, &mut conn).await
    }
}

pub mod like {
    use rocket_db_pools::Connection;
    use crate::model::{Db, Like};
    use crate::users::AuthenticatedUser;
    use crate::api::error::DeductResult;

    #[put("/<graph_id>")]
    pub async fn like_graph(user: AuthenticatedUser, graph_id: uuid::Uuid, mut conn: Connection<Db>) -> DeductResult<()> {
        Like::insert(graph_id, user.db_id, &mut conn).await
    }

    #[delete("/<graph_id>")]
    pub async fn unlike_graph(user: AuthenticatedUser, graph_id: uuid::Uuid, mut conn: Connection<Db>) -> DeductResult<()> {
        Like::delete(graph_id, user.db_id, &mut conn).await
    }
}

pub mod preview {
    use rocket_db_pools::Connection;
    use crate::model::{Db, KnowledgeGraph};
    use rocket::serde::json::Json;
    use crate::api::error::DeductResult;
    
    #[get("/<graph_id>")]
    pub async fn preview(graph_id: uuid::Uuid, mut conn: Connection<Db>) -> DeductResult<Json<KnowledgeGraph>> {
        Ok(Json(KnowledgeGraph::get(graph_id, &mut conn).await?))
    }
}
