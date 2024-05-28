// Contains API endpoints for learning map generation

pub mod objective {
    use crate::api::error::DeductResult;
    use crate::api::users::AuthenticatedUser;
    use crate::model::learning_map::LearningMap;
    use crate::search::SearchResultGraph;
    use rocket::FromForm;
    use rocket::form::Form;
    use rocket_db_pools::Connection;
    use rocket::serde::json::Json;
    use crate::model::{Db, Objective};

    #[derive(FromForm)]
    struct ObjectiveCreation {
        title: String,
        description: String
    }

    #[put("/", data = "<data>")]
    pub async fn create_objective(user: AuthenticatedUser, data: Form<ObjectiveCreation>, mut conn: Connection<Db>) -> DeductResult<()> {
        Objective::create(user, data.title.clone(), data.description.clone(), &mut conn).await
    }

    #[get("/search?<id>&<offset>")]
    pub async fn get_satisfied_graphs(id: i64, offset: i64, mut conn: Connection<Db>) -> DeductResult<Json<Vec<SearchResultGraph>>> {

    }
}

pub mod learning {
    use crate::api::error::DeductResult;
    use crate::api::users::AuthenticatedUser;
    use crate::search::SearchResultGraph;
    use rocket_db_pools::Connection;
    use rocket::serde::json::Json;
    use crate::model::Db;

    #[derive(Deserialize)]
    struct LearningMapCreation {
        pub knowledge_graph: uuid::Uuid,
        pub topic: i64
    }

    #[post("/create", data = "<data>", format = "json")]
    pub async fn create_learning_map(user: AuthenticatedUser, data: Json<LearningMapCreation>, mut conn: Connection<Db>) -> DeductResult<()> {
        
    }

    #[get("/?<page>")]
    pub async fn get_learning_maps(user: AuthenticatedUser, page: Option<i64>, mut conn: Connection<Db>) -> DeductResult<()> {

    }
}