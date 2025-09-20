use crate::application::services_service::ServicesService;
use crate::domain::services::{CreateService, UpdateService};
use crate::infrastructure::services_repository::PgServiceRepository;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

pub type SharedServicesService = Arc<ServicesService<PgServiceRepository>>;

pub async fn get_all(State(service): State<SharedServicesService>) -> impl IntoResponse {
    match service.get_all().await {
        Ok(services) => (StatusCode::OK, Json(services)).into_response(),
        Err(e) => {
            eprintln!("Error al obtener servicios: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response()
        }
    }
}

pub async fn get_by_id(
    Path(id): Path<i32>,
    State(service): State<SharedServicesService>,
) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(s)) => (StatusCode::OK, Json(s)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Servicio no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al buscar servicio").into_response(),
    }
}

pub async fn create(
    State(service): State<SharedServicesService>,
    Json(data): Json<CreateService>,
) -> impl IntoResponse {
    match service.create(data).await {
        Ok(s) => (StatusCode::CREATED, Json(s)).into_response(),
        Err(e) => {
            eprintln!("Error al crear servicio: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al crear servicio").into_response()
        }
    }
}

pub async fn update(
    Path(id): Path<i32>,
    State(service): State<SharedServicesService>,
    Json(data): Json<UpdateService>,
) -> impl IntoResponse {
    // Actualizar servicio 
    match service.update(id, data).await {
        Ok(Some(s)) => (StatusCode::OK, Json(s)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Servicio no encontrado").into_response(),
        Err(e) => {
            eprintln!("Error al actualizar servicio: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error al actualizar servicio",
            )
                .into_response()
        }
    }
}

pub async fn delete(
    Path(id): Path<i32>,
    State(service): State<SharedServicesService>,
) -> impl IntoResponse {
    match service.delete(id).await {
        Ok(Some(s)) => (StatusCode::OK, Json(s)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Servicio no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al eliminar servicio",
        )
            .into_response(),
    }
}       