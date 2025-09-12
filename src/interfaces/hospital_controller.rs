use axum::{extract::{State, Path}, response::IntoResponse, Json, http::StatusCode};
use crate::{application::hospital_service::HospitalService, domain::hospital::CreateHospital};
use std::sync::Arc;

pub type SharedHospitalService = Arc<HospitalService<crate::infrastructure::hospital_repository::PgHospitalRepository>>;

pub async fn get_all(State(service): State<SharedHospitalService>) -> impl IntoResponse {
    match service.get_all().await {
        Ok(hospitales) => (StatusCode::OK, Json(hospitales)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al obtener hospitales").into_response(),
    }
}

pub async fn get_by_id(Path(id): Path<i32>, State(service): State<SharedHospitalService>) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(h)) => (StatusCode::OK, Json(h)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Hospital no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al buscar hospital").into_response(),
    }
}

pub async fn create(State(service): State<SharedHospitalService>, Json(data): Json<CreateHospital>) -> impl IntoResponse {
    match service.create(data).await {
        Ok(h) => (StatusCode::CREATED, Json(h)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al crear hospital").into_response(),
    }
}

pub async fn update(Path(id): Path<i32>, State(service): State<SharedHospitalService>, Json(data): Json<CreateHospital>) -> impl IntoResponse {
    match service.update(id, data).await {
        Ok(Some(h)) => (StatusCode::OK, Json(h)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Hospital no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al actualizar hospital").into_response(),
    }
}

pub async fn delete(Path(id): Path<i32>, State(service): State<SharedHospitalService>) -> impl IntoResponse {
    match service.delete(id).await {
        Ok(Some(h)) => (StatusCode::OK, Json(h)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Hospital no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al eliminar hospital").into_response(),
    }
}
