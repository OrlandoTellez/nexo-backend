use axum::{extract::{State, Path}, response::IntoResponse, Json, http::StatusCode};
use crate::{application::doctor_service::DoctorService, domain::doctor::{CreateDoctor, UpdateDoctor}};
use std::sync::Arc;

pub type SharedDoctorService = Arc<DoctorService<crate::infrastructure::doctor_repository::PgDoctorRepository>>;

pub async fn get_all(State(service): State<SharedDoctorService>) -> impl IntoResponse {
    match service.get_all().await {
        Ok(doctores) => (StatusCode::OK, Json(doctores)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al obtener doctores").into_response(),
    }
}

pub async fn get_by_id(Path(id): Path<i32>, State(service): State<SharedDoctorService>) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(d)) => (StatusCode::OK, Json(d)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Doctor no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al buscar doctor").into_response(),
    }
}

pub async fn create(State(service): State<SharedDoctorService>, Json(data): Json<CreateDoctor>) -> impl IntoResponse {
    match service.create(data).await {
        Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al crear doctor").into_response(),
    }
}

pub async fn update(Path(id): Path<i32>, State(service): State<SharedDoctorService>, Json(data): Json<UpdateDoctor>) -> impl IntoResponse {
    match service.update(id, data).await {
        Ok(Some(d)) => (StatusCode::OK, Json(d)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Doctor no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al actualizar doctor").into_response(),
    }
}

pub async fn delete(Path(id): Path<i32>, State(service): State<SharedDoctorService>) -> impl IntoResponse {
    match service.delete(id).await {
        Ok(Some(d)) => (StatusCode::OK, Json(d)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Doctor no encontrado").into_response(),        
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al eliminar doctor").into_response(),
    }
}       