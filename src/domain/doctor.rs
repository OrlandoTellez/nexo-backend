use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use validator::Validate;
use crate::helpers::validators::validate_phone;

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

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDoctor {
    pub id_area: i32,
    pub id_service: i32,
    pub id_speciality: Option<i32>,
    pub id_user: Option<i32>,

    #[validate(length(min = 2, message = "El nombre debe tener al menos 2 caracteres"))]
    pub first_name: String,

    pub second_name: Option<String>,

    #[validate(length(min = 2, message = "El apellido debe tener al menos 2 caracteres"))]
    pub first_lastname: String,

    pub second_lastname: Option<String>,

    #[validate(custom = "validate_phone")]
    pub phone: Option<String>,

    #[validate(email(message = "Email inválido"))]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateDoctor {
    pub id_area: Option<i32>,
    pub id_service: Option<i32>,
    pub id_speciality: Option<i32>,
    pub id_user: Option<i32>,

    #[validate(length(min = 2, message = "El nombre debe tener al menos 2 caracteres"))]
    pub first_name: Option<String>,

    pub second_name: Option<String>,

    #[validate(length(min = 2, message = "El apellido debe tener al menos 2 caracteres"))]
    pub first_lastname: Option<String>,

    pub second_lastname: Option<String>,

    #[validate(phone(message = "Número de teléfono inválido"))]
    pub phone: Option<String>,

    #[validate(email(message = "Email inválido"))]
    pub email: Option<String>,
}