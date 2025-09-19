use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Patient {
    pub id_patient: i32,
    pub id_user: Option<i32>,            // Puede ser NULL en la tabla
    pub first_name: String,
    pub second_name: Option<String>,     // Puede ser NULL
    pub first_lastname: String,
    pub second_lastname: Option<String>, // Puede ser NULL
    pub address: Option<String>,         // Puede ser NULL
    pub birthdate: NaiveDate,            // DATE
    pub phone: Option<String>,           // VARCHAR(20), puede ser NULL
    pub email: Option<String>,           // VARCHAR(100), puede ser NULL
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePatient {
    pub id_user: Option<i32>, // es auto incrementado por la BD
    pub first_name: String,
    pub second_name: Option<String>,
    pub first_lastname: String,
    pub second_lastname: Option<String>,
    pub address: Option<String>,
    pub birthdate: NaiveDate,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePatient {
    pub id_user: Option<i32>,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub first_lastname: Option<String>,
    pub second_lastname: Option<String>,
    pub address: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub phone: Option<String>,
    pub email: Option<String>,
}
