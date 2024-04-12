#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
#[macro_use] extern crate diesel_async_migrations;
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
            routes::get_graph, routes::create_graph, routes::delete_graph, routes::delete_requirement, routes::delete_topic,
            routes::add_requirement, routes::add_topic])
        .mount("/oauth", routes![oauth::github_callback, oauth::github_login])
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("run_migrations", run_migrations))
        .attach(OAuth2::<crate::api::oauth::GitHubInfo>::fairing("github"))
}

