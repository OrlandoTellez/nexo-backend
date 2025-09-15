use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Patient {
    pub id_patient: i32,
    pub first_name: String,
    pub second_name: String,
    pub first_lastname: String,
    pub second_lastname: String,
    pub birthdate: String,
    pub phone: String,
    pub address: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePatient {
    pub first_name: String,
    pub second_name: String,
    pub first_lastname: String,
    pub second_lastname: String,
    pub birthdate: String,
    pub phone: String,
    pub address: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePatient {
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub first_lastname: Option<String>,
    pub second_lastname: Option<String>,
    pub birthdate: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub email: Option<String>,
}
