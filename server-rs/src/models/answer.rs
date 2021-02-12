use crate::models::user::User;
use crate::utils::DATE_FORMAT;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Queryable)]
pub struct Answer {
    pub id: i32,
    pub content: String,
    pub question: i32,
    pub author: i32,
    pub created_at: DateTime<Utc>,
}

impl Answer {
    pub fn attach(self, author: User) -> AnswerJson {
        AnswerJson {
            id: self.id,
            content: self.content,
            question: self.question,
            author: author.id,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct AnswerJson {
    pub id: i32,
    pub content: String,
    pub question: i32,
    pub author: i32,
    pub created_at: String,
}
