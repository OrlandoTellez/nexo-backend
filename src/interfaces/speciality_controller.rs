use crate::domain::speciality::{CreateSpeciality, UpdateSpeciality};
use crate::application::speciality_service::SpecialityService;
use crate::infrastructure::speciality_repository::PgSpecialityRepository;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::i32;
use std::sync::Arc;

pub type SharedSpecialityService = Arc<SpecialityService<PgSpecialityRepository>>;

pub async fn get_all(State(service): State<SharedSpecialityService>) -> impl IntoResponse {
    match service.get_all().await {
        Ok(speciality) => (StatusCode::OK, Json(speciality)).into_response(),
        Err(e) => {
            eprintln!("Error al obtener especialidad: {:?}", e);    
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response()
        }
    }
}

pub async fn get_by_id(Path(id): Path<i32>, State(service): State<SharedSpecialityService>) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(speciality)) => (StatusCode::OK, Json(speciality)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Especialidad no encontrada").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al buscar especialidad").into_response() 
    }
}

pub async fn create(State(service): State<SharedSpecialityService>, Json(data): Json<CreateSpeciality>) -> impl IntoResponse {
    match service.create(data).await {
        Ok(speciality) => (StatusCode::CREATED, Json(speciality)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al crear especialidad").into_response()
    }
}

pub async fn update(
    Path(id): Path<i32>, 
    State(service): State<SharedSpecialityService>, 
    Json(data): Json<UpdateSpeciality>
) -> impl IntoResponse {
    match service.update(id, data).await {
        Ok(Some(speciality)) => (StatusCode::OK, Json(speciality)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Especialidad no encontrada").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al actualizar datos").into_response(),
    } 
}

pub async fn delete(Path(id): Path<i32>, State(service): State<SharedSpecialityService>) -> impl IntoResponse {
    match service.delete(id).await{
        Ok(Some(speciality)) => (StatusCode::OK, Json(speciality)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "No se encontro la especialidad").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al eliminar especialidad").into_response(),
    }
} 