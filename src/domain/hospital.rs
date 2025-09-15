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
