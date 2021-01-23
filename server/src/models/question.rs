use crate::DbConn;
use rocket::{get, post, put, routes};
use rocket_contrib::json::{Json, JsonValue};

#[get("/questions")]
pub fn get_all_questions() -> JsonValue {
    todo!();
}
