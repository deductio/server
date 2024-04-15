#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
#[macro_use] extern crate diesel_async_migrations;
extern crate diesel_full_text_search;
extern crate dotenvy;
extern crate reqwest;
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
        .mount("/graph", routes![
            graph::get_graph, graph::create_graph, graph::delete_graph, graph::delete_requirement, graph::delete_topic,
            graph::add_requirement, graph::add_topic])
        .mount("/login", routes![users::github_login])
        .mount("/auth", routes![users::github_callback])
        .mount("/search", routes![search::search_graph])
        .mount("/users", routes![users::view_user])
        .mount("/logout", routes![users::logout])
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("run_migrations", run_migrations))
        .attach(OAuth2::<crate::api::users::GitHubInfo>::fairing("github"))
}

