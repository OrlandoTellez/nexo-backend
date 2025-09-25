use crate::{
    application::lab_result_service::LabResultService,
    domain::lab_result::{CreateLabResult, UpdateLabResult},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use validator::Validate;

pub type SharedLabResultService =
    Arc<LabResultService<crate::infrastructure::lab_result::PgLabResultRepository>>;

pub async fn get_all(State(service): State<SharedLabResultService>) -> impl IntoResponse {
    match service.get_all().await {
        Ok(lab_results) => (StatusCode::OK, Json(lab_results)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al obtener lab_results",
        )
            .into_response(),
    }
}

pub async fn get_by_id(
    Path(id): Path<i32>,
    State(service): State<SharedLabResultService>,
) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(l)) => (StatusCode::OK, Json(l)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Lab result no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al buscar lab result",
        )
            .into_response(),
    }
}

pub async fn create(
    State(service): State<SharedLabResultService>,
    Json(payload): Json<CreateLabResult>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(format!("Errores de validación: {:?}", errors)),
        )
        .into_response();
    }

    match service.create(payload).await {
        Ok(l) => (StatusCode::CREATED, Json(l)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al crear lab result",
        )
            .into_response(),
    }
}


pub async fn update(
    State(service): State<SharedLabResultService>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateLabResult>, 
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {  
        return (
            StatusCode::BAD_REQUEST,
            Json(format!("Errores de validación: {:?}", errors)),
        )
        .into_response();
    }

    match service.update(id, payload).await { 
        Ok(Some(l)) => (StatusCode::OK, Json(l)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Lab result no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al actualizar lab result",
        )
            .into_response(),
    }
}

pub async fn delete(
    Path(id): Path<i32>,
    State(service): State<SharedLabResultService>,
) -> impl IntoResponse {
    match service.delete(id).await {
        Ok(Some(l)) => (StatusCode::OK, Json(l)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Lab result no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al eliminar lab result",
        )
            .into_response(),
    }
}   