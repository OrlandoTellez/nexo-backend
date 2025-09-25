use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct MedicalHistory {
    pub id_history: i32,
    pub id_patient: i32,
    pub id_doctor: Option<i32>,
    pub diagnosis: String,
    pub treatment: Option<String>,
    pub notes: Option<String>,
    pub record_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMedicalHistory {
    pub id_patient: i32,
    pub id_doctor: Option<i32>,
    #[validate(length(min = 3, message = "Diagnóstico muy corto"))]
    pub diagnosis: String,
    pub treatment: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMedicalHistory {
    pub diagnosis: Option<String>,
    pub treatment: Option<String>,
    pub notes: Option<String>,
}
