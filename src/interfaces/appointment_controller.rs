use crate::{
    application::appointment_service::AppointmentService,
    domain::appointment::{CreateAppointment, UpdateAppointment},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use validator::Validate;

pub type SharedAppointmentService =
    Arc<AppointmentService<crate::infrastructure::appointment_repository::PgAppointmentRepository>>;

pub async fn get_all(State(service): State<SharedAppointmentService>) -> impl IntoResponse {
    match service.get_all().await {
        Ok(appointments) => (StatusCode::OK, Json(appointments)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al obtener citas médicas",
        )
            .into_response(),
    }
}

pub async fn get_by_id(
    Path(id): Path<i32>,
    State(service): State<SharedAppointmentService>,
) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(a)) => (StatusCode::OK, Json(a)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Cita médica no encontrada").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al buscar cita médica",
        )
            .into_response(),
    }
}

pub async fn create(
    State(service): State<SharedAppointmentService>,
    Json(payload): Json<CreateAppointment>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(format!("Errores de validación: {:?}", errors)),
        )
        .into_response();
    }

    match service.create(payload).await {
        Ok(a) => (StatusCode::CREATED, Json(a)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al crear cita médica",
        )
            .into_response(),
    }
}

pub async fn update(
    State(service): State<SharedAppointmentService>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateAppointment>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(format!("Errores de validación: {:?}", errors)),
        )
        .into_response();
    }

    match service.update(id, payload).await {
        Ok(Some(a)) => (StatusCode::OK, Json(a)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Cita médica no encontrada").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al actualizar cita médica",
        )
            .into_response(),
    }
}

pub async fn delete(
    Path(id): Path<i32>,
    State(service): State<SharedAppointmentService>,
) -> impl IntoResponse {
    match service.delete(id).await {
        Ok(Some(a)) => (StatusCode::OK, Json(a)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Cita médica no encontrada").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al eliminar cita médica",
        )
            .into_response(),
    }
}   