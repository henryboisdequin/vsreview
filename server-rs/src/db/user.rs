use crate::diesel::query_dsl::filter_dsl::FilterDsl;
use crate::diesel::query_dsl::filter_dsl::FindDsl;
use crate::diesel::ExpressionMethods;
use crate::error::AuthErr;
use crate::models::user::*;
use crate::schema::*;
use crate::utils::get_env;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use diesel::PgConnection;
use diesel::RunQueryDsl;
use rocket::http::private::CookieJar;
use rocket::http::Cookie;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub bio: String,
    pub profile_image: String,
}

pub fn create(
    mut cookies: &CookieJar,
    registering_user: User,
    connection: &PgConnection,
) -> Result<User, AuthErr<'static>> {
    let new_user = NewUser {
        email: registering_user.email,
        username: registering_user.username,
        password_hash: hash_password(registering_user.password_hash).unwrap(),
        bio: registering_user.bio,
        profile_image: registering_user.profile_image,
    };
    let session_secret = get_env("SESSION_SECRET");

    // add cookie
    cookies.add(Cookie::new(session_secret, new_user.username));

    match diesel::insert_into(user::table)
        .values(new_user)
        .get_result::<User>(connection)
    {
        Err(e) => Err(AuthErr::Other(e)),
        Ok(val) => Ok(val),
    }
}

pub fn get_current_user(conn: &PgConnection, cookies: &CookieJar) -> Option<User> {
    match cookies.get(&get_env("SESSION_SECRET").as_str()) {
        Some(username) => {
            let user = user::table
                .filter(user::username.eq(username.value()))
                .first(conn)
                .unwrap();
            Some(user)
        }
        None => None,
    }
}

pub fn hash_password(plain: String) -> Result<String, BcryptError> {
    Ok(hash(plain, DEFAULT_COST)?)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    // user can log in with their email or username
    pub username: String,
    pub password: String,
}

pub fn login(conn: &PgConnection, user_info: Login) -> Option<User> {
    let user = user::table
        .filter(user::username.eq(user_info.username))
        .get_result::<User>(conn)
        .map_err(|err| eprintln!("{}", err))
        .ok()?;
    let is_valid_password = verify(user_info.password, &user.password_hash).ok()?;

    if is_valid_password {
        Some(user)
    } else {
        eprintln!("Error logging in: {}", user_info.username);
        None
    }
}

#[derive(Serialize, Deserialize, Debug, AsChangeset, Default)]
#[table_name = "user"]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub profile_image: Option<String>,
}

pub fn update(conn: &PgConnection, username: String, data: &UpdateUser) -> Option<User> {
    let data = &UpdateUser { ..*data.clone() };
    diesel::update(user::table.find(username))
        .set(data)
        .get_result(conn)
        .ok()
}
