// Contains API endpoints for learning map generation

pub mod objective {
    use crate::api::error::DeductResult;
    use crate::api::users::AuthenticatedUser;
    use crate::search::SearchResultGraph;
    use rocket::FromForm;
    use rocket::form::Form;
    use rocket_db_pools::Connection;
    use rocket::serde::json::Json;
    use crate::model::{Db, Objective};

    #[derive(FromForm)]
    pub struct ObjectiveCreation {
        title: String,
        description: String
    }

    #[derive(FromForm)]
    pub struct ObjectiveSearch {
        search: String
    }

    #[put("/", data = "<data>")]
    pub async fn create_objective(user: AuthenticatedUser, data: Form<ObjectiveCreation>, mut conn: Connection<Db>) -> DeductResult<()> {
        Objective::create(user, data.title.clone(), data.description.clone(), &mut conn).await
    }

    #[get("/search?<id>&<offset>")]
    pub async fn get_satisfied_graphs(_user: AuthenticatedUser, id: i64, offset: i64, mut conn: Connection<Db>) -> DeductResult<Json<Vec<SearchResultGraph>>> {
        Ok(Json(Objective::get_satisfied_graphs(id, offset, &mut conn).await?))
    }

    #[post("/?<page>", data = "<form>")]
    pub async fn search_objectives(_user: AuthenticatedUser, page: Option<i64>, form: Form<ObjectiveSearch>, mut conn: Connection<Db>) -> DeductResult<Json<Vec<Objective>>> {
        Ok(Json(Objective::search_objectives(form.search.clone(), page.unwrap_or(0), &mut conn).await?))
    }
}

pub mod learning {
    use crate::error::DeductError;
    use crate::model::learning_map::SimpleLearningMap;
    use crate::{api::error::DeductResult, model::learning_map::ResponseLearningMap};
    use crate::api::users::AuthenticatedUser;
    use rocket_db_pools::Connection;
    use rocket::serde::json::Json;
    use crate::model::{LearningMap, Db};

    #[derive(Deserialize)]
    pub struct LearningMapCreation {
        pub knowledge_graph_id: uuid::Uuid,
        pub topic: i64,
        pub name: String
    }

    #[post("/create", data = "<data>", format = "json")]
    pub async fn create_learning_map(user: AuthenticatedUser, data: Json<LearningMapCreation>, mut conn: Connection<Db>) -> DeductResult<Json<ResponseLearningMap>> {
        Ok(Json(LearningMap::generate(user, (*data).name.clone(), (*data).topic, &mut conn).await?))
    }

    #[get("/?<page>")]
    pub async fn get_learning_maps(user: AuthenticatedUser, page: Option<i64>, mut conn: Connection<Db>) -> DeductResult<Json<Vec<SimpleLearningMap>>> {
        Ok(Json(LearningMap::get_learning_maps(user, page.unwrap_or(0), &mut conn).await?))
    }

    #[get("/<id>")]
    pub async fn get_learning_map(user: AuthenticatedUser, id: i64, mut conn: Connection<Db>) -> DeductResult<Json<ResponseLearningMap>> {
        let learning_map = LearningMap::get(id, &mut conn).await?;

        if learning_map.user_id != user.db_id {
            Err(DeductError::UnauthorizedUser("User is not the owner of the given learning map"))
        } else {
            Ok(Json(learning_map.to_response(&mut conn).await?))
        }
    }
}
