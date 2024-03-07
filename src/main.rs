#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
extern crate dotenvy;
extern crate diesel;
extern crate uuid;

mod api;
mod model;
mod schema;

use crate::model::Db;
use crate::api::*;
use rocket_db_pools::Database;

#[get("/")]
fn scrub() -> &'static str {
    "waga waga!"
}

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok().unwrap();

    rocket::build()
        .mount("/", routes![scrub])
        .mount("/graph", routes![routes::get_graph, routes::get_topic_resources])
        .attach(Db::init())
}

