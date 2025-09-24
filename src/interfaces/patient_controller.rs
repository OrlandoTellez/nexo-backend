use axum::{extract::{State, Path}, response::IntoResponse, Json, http::StatusCode};
use crate::application::patient_service::PatientService;
use crate::domain::patient::{CreatePatient, UpdatePatient};
use crate::infrastructure::patient_repository::PgPatientRepository;
use std::sync::Arc;
use validator::Validate;


pub type SharedPatientService = Arc<PatientService<PgPatientRepository>>;

pub async fn get_all(State(service): State<SharedPatientService> ) -> impl IntoResponse {
    match service.get_all().await {
        Ok(patients) => (StatusCode::OK, Json(patients)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al obtener pacientes").into_response()
    }
}

pub async fn get_by_id(Path(id): Path<i32>, State(service): State<SharedPatientService>) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(p)) => (StatusCode::OK, Json(p)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Patiente no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al buscar paciente").into_response(),
    }
}


pub async fn create(
    State(service): State<SharedPatientService>,
    Json(payload): Json<CreatePatient>,
) -> impl IntoResponse {
    // Validamos el payload antes de enviarlo al servicio
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(format!("Errores de validación: {:?}", errors)),
        )
            .into_response();
    }

    match service.create(payload).await {
        Ok(patient) => (StatusCode::CREATED, Json(patient)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al crear paciente",
        )
            .into_response(),
    }
}
pub async fn update(Path(id): Path<i32>, State(service): State<SharedPatientService>, Json(data): Json<UpdatePatient>) -> impl IntoResponse {
    match service.update(id, data).await {
        Ok(Some(p)) => (StatusCode::OK, Json(p)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Patiente no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al actualizar paciente").into_response(),
    }
}

pub async fn delete(Path(id): Path<i32>, State(service): State<SharedPatientService>) -> impl IntoResponse {
    match service.delete(id).await {
        Ok(Some(p)) => (StatusCode::OK, Json(p)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Patiente no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al eliminar paciente").into_response(),
    }
} 