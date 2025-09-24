use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate, NaiveDateTime};
use validator::{Validate};
use crate::helpers::validators::validate_phone;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Patient {
    pub id_patient: i32,
    pub id_user: Option<i32>,
    pub first_name: String,
    pub second_name: Option<String>,
    pub first_lastname: String,
    pub second_lastname: Option<String>,
    pub address: Option<String>,
    pub birthdate: NaiveDate,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePatient {
    pub id_user: Option<i32>,

    #[validate(length(min = 2, message = "El nombre debe tener al menos 2 caracteres"))]
    pub first_name: String,

    pub second_name: Option<String>,

    #[validate(length(min = 2, message = "El apellido debe tener al menos 2 caracteres"))]
    pub first_lastname: String,

    pub second_lastname: Option<String>,

    pub address: Option<String>,

    pub birthdate: NaiveDate,

    #[validate(custom = "validate_phone")]
    pub phone: Option<String>,

    #[validate(email(message = "Email inválido"))]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePatient {
    pub id_user: Option<i32>,

    #[validate(length(min = 2, message = "El nombre debe tener al menos 2 caracteres"))]
    pub first_name: Option<String>,

    pub second_name: Option<String>,

    #[validate(length(min = 2, message = "El apellido debe tener al menos 2 caracteres"))]
    pub first_lastname: Option<String>,

    pub second_lastname: Option<String>,

    pub address: Option<String>,

    pub birthdate: Option<NaiveDate>,

    pub phone: Option<String>,

    #[validate(email(message = "Email inválido"))]
    pub email: Option<String>,
}


