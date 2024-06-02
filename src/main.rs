#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
#[macro_use] extern crate diesel_async_migrations;
extern crate diesel_full_text_search;
extern crate futures_concurrency;
extern crate dotenvy;
extern crate reqwest;
extern crate chrono;
extern crate diesel;
extern crate uuid;

mod api;
mod model;
mod schema;

use crate::model::Db;
use crate::api::*;
use diesel_async_migrations::EmbeddedMigrations;
use rocket_db_pools::Database;
use rocket::{Rocket, Build};
use rocket::fairing::{AdHoc, self};
use rocket_oauth2::OAuth2;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    if let Some(db) = Db::fetch(&rocket) {
        if let Ok(mut connection) = (&db.0).get().await {
            match MIGRATIONS.run_pending_migrations(&mut connection).await {
                Ok(_) => Ok(rocket),
                Err(_) => Err(rocket)
            }
        } else {
            Err(rocket)
        }
    } else {
        Err(rocket)
    }
}

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok().unwrap();

    rocket::build()
        .mount("/api/graph/view", routes![graph::view::get_graph_from_username, graph::view::get_graph])
        .mount("/api/graph/edit", routes![graph::edit::add_requirement, graph::edit::add_topic, graph::edit::delete_requirement,
            graph::edit::delete_topic, graph::edit::delete_graph, graph::edit::modify_graph_info, graph::edit::add_objective_prerequisite,
            graph::edit::delete_objective_prerequisite, graph::edit::add_objective_satisfier, graph::edit::delete_objective_satisfier, 
            graph::edit::start_editing_graph])
        .mount("/api/graph/progress", routes![graph::progress::put_progress, graph::progress::delete_progress])
        .mount("/api/graph/create", routes![graph::create::create_graph])
        .mount("/api/graph/like", routes![graph::like::like_graph, graph::like::unlike_graph])
        .mount("/api/login", routes![users::github_login])
        .mount("/api/auth", routes![users::github_callback])
        .mount("/api/search", routes![search::search_graph])
        .mount("/api/users", routes![users::view_user])
        .mount("/api/logout", routes![users::logout])
        .mount("/api/trending", routes![search::get_trending_graphs])
        .mount("/api/graph/preview", routes![graph::preview::preview])
        .mount("/api/objectives", routes![planning::objective::get_satisfied_graphs, planning::objective::create_objective, planning::objective::search_objectives])
        .mount("/api/maps", routes![planning::learning::create_learning_map, planning::learning::get_learning_map, planning::learning::get_learning_maps])
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("run_migrations", run_migrations))
        .attach(OAuth2::<crate::api::users::GitHubInfo>::fairing("github"))
}

