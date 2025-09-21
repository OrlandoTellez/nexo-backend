use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Speciality {
    pub id_speciality: i32,
    pub speciality_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>
}

#[derive(Debug, Deserialize)]
pub struct CreateSpeciality {
    pub speciality_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSpeciality {
    pub speciality_name: String
}