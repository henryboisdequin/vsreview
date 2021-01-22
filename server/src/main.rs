#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

mod schema;

use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use rocket::response::content::Html;
use rocket::{get, post, put, routes};
use rocket_contrib::databases::{database, diesel::PgConnection};
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};

#[database("postgres")]
struct DbConn(PgConnection);

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "404 not found."
    })
}

#[get("/")]
fn index() -> Html<&'static str> {
    Html(r"<h1>VSReview API - Version: 0.1.0</h1>")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .register(catchers![not_found])
        .launch();
}
