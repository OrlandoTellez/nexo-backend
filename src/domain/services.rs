use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Service {
    id_service: i32,
    service_name: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
    deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateService {
    pub service_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateService {
    pub service_name: Option<String>,
}