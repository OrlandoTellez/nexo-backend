use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Appointment {
    pub id_appointment: i32,
    pub id_patient: i32,
    pub id_doctor: i32,
    pub id_area: i32,
    pub id_service: i32,
    pub appointment_datetime: NaiveDateTime,
    pub building: Option<String>,
    pub room: Option<String>,
    pub notes: Option<String>,
    pub prescription: Option<String>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAppointment {
    pub id_patient: i32,
    pub id_doctor: i32,
    pub id_area: i32,
    pub id_service: i32,

    pub appointment_datetime: NaiveDateTime,

    pub building: Option<String>,
    pub room: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAppointment {
    pub appointment_datetime: Option<NaiveDateTime>,
    pub building: Option<String>,
    pub room: Option<String>,
    pub notes: Option<String>,
    pub prescription: Option<String>,
    pub status: Option<String>, // pending, confirmed, completed, canceled
}
