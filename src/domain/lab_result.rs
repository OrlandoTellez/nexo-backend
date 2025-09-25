use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct LabResult {
    pub id_result: i32,
    pub id_patient: i32,
    pub id_doctor: Option<i32>,
    pub lab_name: String,
    pub test_type: Option<String>,
    pub result: String,
    pub result_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLabResult {
    pub id_patient: i32,
    pub id_doctor: Option<i32>,
    pub lab_name: String,
    pub test_type: Option<String>,
    #[validate(length(min = 2, message = "Resultado inválido"))]
    pub result: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateLabResult {
    pub lab_name: Option<String>,
    pub test_type: Option<String>,
    pub result: Option<String>,
}
