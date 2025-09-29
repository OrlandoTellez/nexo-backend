use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate, NaiveDateTime};
use validator::{Validate};
use crate::helpers::validators::validate_phone;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Patient {
    pub id_patient: i32,
    pub id_user: Option<i32>,
    pub identity_number: String,  // Cédula

    pub first_name: String,
    pub second_name: Option<String>,
    pub first_lastname: String,
    pub second_lastname: Option<String>,

    pub gender: Option<String>, // 'M', 'F', 'O'
    pub birthdate: NaiveDate,
    pub blood_type: Option<String>, // A+, O-, etc.
 
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
 
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
 
    pub allergies: Option<String>,
    pub current_medications: Option<String>,
    pub medical_background: Option<String>,
 
    pub priority: Option<i32>,
    pub status: Option<String>,
 
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePatient {
    #[validate(length(min = 8, message = "La contraseña debe tener al menos 8 caracteres"))]
 
    #[validate(length(min = 5, message = "La cédula debe tener al menos 5 caracteres"))]
    pub identity_number: String,

    #[validate(length(min = 2, message = "El nombre debe tener al menos 2 caracteres"))]
    pub first_name: String,
    pub second_name: Option<String>,

    #[validate(length(min = 2, message = "El apellido debe tener al menos 2 caracteres"))]
    pub first_lastname: String,
    pub second_lastname: Option<String>,

    #[validate(length(equal = 1, message = "El género debe ser M, F u O"))]
    pub gender: Option<String>,

    pub birthdate: NaiveDate,

    #[validate(length(min = 2, max = 3, message = "Tipo de sangre inválido"))]
    pub blood_type: Option<String>,

    #[validate(custom = "validate_phone")]
    pub phone: Option<String>,

    #[validate(email(message = "Email inválido"))]
    pub email: Option<String>,

    pub address: Option<String>,

    pub emergency_contact_name: Option<String>,
    #[validate(custom = "validate_phone")]
    pub emergency_contact_phone: Option<String>,

    pub allergies: Option<String>,
    pub current_medications: Option<String>,
    pub medical_background: Option<String>,

    pub priority: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePatient {
    pub id_user: Option<i32>,

    #[validate(length(min = 5, message = "La cédula debe tener al menos 5 caracteres"))]
    pub identity_number: Option<String>,

    #[validate(length(min = 2, message = "El nombre debe tener al menos 2 caracteres"))]
    pub first_name: Option<String>,
    pub second_name: Option<String>,

    #[validate(length(min = 2, message = "El apellido debe tener al menos 2 caracteres"))]
    pub first_lastname: Option<String>,
    pub second_lastname: Option<String>,

    pub gender: Option<String>,
    pub birthdate: Option<NaiveDateTime>,
    pub blood_type: Option<String>,

    #[validate(custom = "validate_phone")]
    pub phone: Option<String>,

    #[validate(email(message = "Email inválido"))]
    pub email: Option<String>,

    pub address: Option<String>,
    pub emergency_contact_name: Option<String>,
    #[validate(custom = "validate_phone")]
    pub emergency_contact_phone: Option<String>,

    pub allergies: Option<String>,
    pub current_medications: Option<String>,
    pub medical_background: Option<String>,

    pub priority: Option<i32>,
    pub status: Option<String>,
}

