use crate::{
    application::doctor_service::DoctorService,
    domain::doctor::{CreateDoctor, UpdateDoctor},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use validator::Validate;

pub type SharedDoctorService =
    Arc<DoctorService<crate::infrastructure::doctor_repository::PgDoctorRepository>>;

pub async fn get_all(State(service): State<SharedDoctorService>) -> impl IntoResponse {
    match service.get_all().await {
        Ok(doctores) => (StatusCode::OK, Json(doctores)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al obtener doctores",
        )
            .into_response(),
    }
}

pub async fn get_by_id(
    Path(id): Path<i32>,
    State(service): State<SharedDoctorService>,
) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(d)) => (StatusCode::OK, Json(d)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Doctor no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al buscar doctor").into_response(),
    }
}

pub async fn create(
    State(service): State<SharedDoctorService>,
    Json(payload): Json<CreateDoctor>,
) -> impl IntoResponse {
    // aquí payload ya es CreateDoctor
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(format!("Errores de validación: {:?}", errors)),
        )
        .into_response();
    }

    match service.create(payload).await {
        Ok(doc) => (StatusCode::CREATED, Json(doc)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al crear doctor",
        )
        .into_response(),
    }
}


pub async fn update(
    State(service): State<SharedDoctorService>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateDoctor>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(format!("Errores de validación: {:?}", errors)),
        )
            .into_response();
    }

    match service.update(id, payload).await {
        Ok(Some(doc)) => (StatusCode::OK, Json(doc)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Doctor no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al actualizar doctor",
        )
            .into_response(),
    }
}
pub async fn delete(
    Path(id): Path<i32>,
    State(service): State<SharedDoctorService>,
) -> impl IntoResponse {
    match service.delete(id).await {
        Ok(Some(d)) => (StatusCode::OK, Json(d)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Doctor no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al eliminar doctor",
        )
            .into_response(),
    }
}
