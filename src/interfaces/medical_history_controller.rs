use crate::{
    application::medical_history_service::MedicalHistoryService,
    domain::medical_history::{CreateMedicalHistory, UpdateMedicalHistory},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use validator::Validate;

pub type SharedMedicalHistoryService =
    Arc<MedicalHistoryService<crate::infrastructure::medical_history_repository::PgMedicalHistoryRepository>>;

pub async fn get_all(State(service): State<SharedMedicalHistoryService>) -> impl IntoResponse {
    match service.get_all().await {
        Ok(medical_histories) => (StatusCode::OK, Json(medical_histories)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al obtener medical histories",
        )
            .into_response(),
    }
}

pub async fn get_by_id(
    Path(id): Path<i32>,
    State(service): State<SharedMedicalHistoryService>,
) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(m)) => (StatusCode::OK, Json(m)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Medical history no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al buscar medical history",
        )
            .into_response(),
    }
}

pub async fn create(
    State(service): State<SharedMedicalHistoryService>,
    Json(payload): Json<CreateMedicalHistory>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(format!("Errores de validación: {:?}", errors)),
        )
        .into_response();
    }

    match service.create(payload).await {
        Ok(m) => (StatusCode::CREATED, Json(m)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al crear medical history",
        )
            .into_response(),
    }
}


pub async fn update(
    State(service): State<SharedMedicalHistoryService>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateMedicalHistory>, 
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {  
        return (
            StatusCode::BAD_REQUEST,
            Json(format!("Errores de validación: {:?}", errors)),
        )
        .into_response();
    }

    match service.update(id, payload).await { 
        Ok(Some(m)) => (StatusCode::OK, Json(m)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Medical history no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al actualizar medical history",
        )
            .into_response(),
    }
}

pub async fn delete(
    Path(id): Path<i32>,
    State(service): State<SharedMedicalHistoryService>,
) -> impl IntoResponse {
    match service.delete(id).await {
        Ok(Some(m)) => (StatusCode::OK, Json(m)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Medical history no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al eliminar medical history",
        )
            .into_response(),
    }
}   