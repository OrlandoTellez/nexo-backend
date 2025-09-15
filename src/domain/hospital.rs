use serde::{Serialize, Deserialize};
use sqlx::FromRow; 

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Hospital {
    pub id_hospital: i32,
    pub name: String,
    pub address: String
}

#[derive(Debug, Deserialize)]
pub struct CreateHospital {
    pub name: String,
    pub address: String
}

#[derive(Debug, Serialize, Deserialize)]
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