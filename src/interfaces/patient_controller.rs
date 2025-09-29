use crate::application::patient_service::PatientService;
use crate::domain::patient::{CreatePatient, UpdatePatient};
use crate::helpers::utils::generate_patient_password;
use crate::infrastructure::patient_repository::PgPatientRepository;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Datelike; // para el método `.year()`

use std::sync::Arc;
use validator::Validate;

pub type SharedPatientService = Arc<PatientService<PgPatientRepository>>;

pub async fn get_all(State(service): State<SharedPatientService>) -> impl IntoResponse {
    match service.get_all().await {
        Ok(patients) => (StatusCode::OK, Json(patients)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al obtener pacientes",
        )
            .into_response(),
    }
}

pub async fn get_by_id(
    Path(id): Path<i32>,
    State(service): State<SharedPatientService>,
) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(p)) => (StatusCode::OK, Json(p)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Patiente no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al buscar paciente",
        )
            .into_response(),
    }
}

pub async fn create(
    State(service): State<SharedPatientService>,
    Json(payload): Json<CreatePatient>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(format!("Errores de validación: {:?}", errors)),
        )
            .into_response();
    }

    let birth_year = payload.birthdate.year(); 
    let raw_password =
        generate_patient_password(&payload.first_name, &payload.first_lastname, birth_year);

    // Pasamos al servicio
    match service.create(payload, &raw_password).await {
        Ok(patient) => (StatusCode::CREATED, Json(patient)).into_response(),
        Err(e) => {
            eprintln!("Error al crear paciente: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al crear paciente").into_response()
        }
    }
}

pub async fn update(
    Path(id): Path<i32>,
    State(service): State<SharedPatientService>,
    Json(data): Json<UpdatePatient>,
) -> impl IntoResponse {
    match service.update(id, data).await {
        Ok(Some(p)) => (StatusCode::OK, Json(p)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Patiente no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al actualizar paciente",
        )
            .into_response(),
    }
}

pub async fn delete(
    Path(id): Path<i32>,
    State(service): State<SharedPatientService>,
) -> impl IntoResponse {
    match service.delete(id).await {
        Ok(Some(p)) => (StatusCode::OK, Json(p)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Patiente no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al eliminar paciente",
        )
            .into_response(),
    }
}
