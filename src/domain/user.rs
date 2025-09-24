use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use validator::{Validate};
use crate::helpers::validators::validate_role;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id_user: i32,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, message = "El username debe tener al menos 3 caracteres"))]
    pub username: String,

    #[validate(length(min = 6, message = "La contraseña debe tener al menos 6 caracteres"))]
    pub password_hash: String,

    #[validate(custom = "validate_role")]
    pub role: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(min = 3, message = "El username debe tener al menos 3 caracteres"))]
    pub username: Option<String>,

    #[validate(length(min = 6, message = "La contraseña debe tener al menos 6 caracteres"))]
    pub password_hash: Option<String>,

    #[validate(custom = "validate_role")]
    pub role: Option<String>,
}

