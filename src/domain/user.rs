use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;


#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id_user: i32,
    pub username: String,
    pub password_hash: String,
    pub role: String, // 'patient', 'doctor', 'admin'
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password_hash: Option<String>,
    pub role: Option<String>,
}

