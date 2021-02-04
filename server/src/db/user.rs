use crate::error::AuthErr;
use crate::models::user::*;
use crate::schema::*;
use bcrypt::{hash, BcryptError, DEFAULT_COST};
use diesel::PgConnection;
use diesel::RunQueryDsl;
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

impl User {
    pub fn create(
        registering_user: User,
        connection: &PgConnection,
    ) -> Result<User, AuthErr<'static>> {
        // todo: find better way to get mystery man profile pic
        let mystery_man =
            "https://cdn.pixabay.com/photo/2015/10/05/22/37/blank-profile-picture-973460_1280.png"
                .to_string();
        let profile_image = match registering_user.profile_image {
            None => mystery_man,
            Some(val) => val,
        };

        Ok(diesel::insert_into(user::table)
            .values(NewUser {
                email: registering_user.email,
                username: registering_user.username,
                password_hash: Self::hash_password(registering_user.password_hash).unwrap(),
                bio: registering_user.bio,
                profile_image,
            })
            .get_result(connection)
            .unwrap())
    }

    pub fn hash_password(plain: String) -> Result<String, BcryptError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }
}

pub struct Register {
    pub email: String,
    pub username: String,
    pub password: String,
    pub password_confirmation: String,
    pub bio: String,
    pub profile_image: String,
}

impl Register {
    pub fn register_user(self) -> Result<Register, AuthErr<'static>> {
        if self.password == self.password_confirmation {
            Ok(self)
        } else {
            Err(AuthErr::PasswordNotMatch(
                "Password and password confirmation do not match.",
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    // user can log in with their email or username
    pub email_or_username: String,
    pub password: String,
}

impl Login {}
