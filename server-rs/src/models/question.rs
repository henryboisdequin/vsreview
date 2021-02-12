use crate::models::user::User;
use crate::utils::DATE_FORMAT;
use chrono::{DateTime, Utc};
use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable)]
pub struct Question {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tag_list: Vec<String>,
    pub author: i32,
    pub created_at: DateTime<Utc>,
}

impl Question {
    pub fn attach(self, author: User) -> QuestionJson {
        QuestionJson {
            id: self.id,
            title: self.title,
            content: self.content,
            tag_list: self.tag_list,
            author: author.id,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct QuestionJson {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tag_list: Vec<String>,
    pub author: i32,
    pub created_at: String,
}
