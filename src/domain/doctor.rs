use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;


#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Doctor {
    pub id_doctor: i32,
    pub id_area: i32,
    pub id_speciality: Option<i32>,
    pub id_service: i32,
    pub id_user: Option<i32>,
    pub first_name: String,
    pub second_name: Option<String>,
    pub first_lastname: String,
    pub second_lastname: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDoctor {
    pub id_area: i32,
    pub id_service: i32,
    pub id_speciality: Option<i32>,
    pub id_user: Option<i32>,
    pub first_name: String,
    pub second_name: Option<String>,
    pub first_lastname: String,
    pub second_lastname: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDoctor {
    pub id_area: Option<i32>,
    pub id_service: Option<i32>,
    pub id_speciality: Option<i32>,
    pub id_user: Option<i32>,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub first_lastname: Option<String>,
    pub second_lastname: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}   