use serde::Serialize;

type Url = String;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub bio: String,
    pub profile_image: Option<Url>,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

#[derive(Serialize)]
pub struct Profile {
    username: String,
    bio: String,
    profile_image: Option<Url>,
}

impl User {
    pub fn to_profile(self) -> Profile {
        Profile {
            username: self.username,
            bio: self.bio,
            profile_image: self.profile_image,
        }
    }
}
