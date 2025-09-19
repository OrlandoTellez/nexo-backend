use axum::{extract::{State, Path}, response::IntoResponse, Json, http::StatusCode};
use crate::domain::user::{User, CreateUser, UpdateUser};
use crate::application::user_service::UserService;
use crate::infrastructure::user_repository::PgUserRepository;
use std::sync::Arc;

pub type SharedUserService = Arc<UserService<PgUserRepository>>;

pub async fn get_all(State(service): State<SharedUserService> ) -> impl IntoResponse {
    match service.get_all().await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al obtener usuarios").into_response()
    }
}

pub async fn get_by_id(Path(id): Path<i32>, State(service): State<SharedUserService>) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(u)) => (StatusCode::OK, Json(u)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Usuario no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al buscar usuario").into_response(),
    }
}

pub async fn create(State(service): State<SharedUserService>, Json(data): Json<CreateUser>) -> impl IntoResponse {
    match service.create(data).await {
        Ok(u) => (StatusCode::CREATED, Json(u)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al crear usuario").into_response(),
    }
}

pub async fn update(Path(id): Path<i32>, State(service): State<SharedUserService>, Json(data): Json<UpdateUser>) -> impl IntoResponse {
    match service.update(id, data).await {
        Ok(Some(u)) => (StatusCode::OK, Json(u)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Usuario no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al actualizar usuario").into_response(),
    }
}

pub async fn delete(Path(id): Path<i32>, State(service): State<SharedUserService>) -> impl IntoResponse {
    match service.delete(id).await {
        Ok(Some(u)) => (StatusCode::OK, Json(u)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Usuario no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al eliminar usuario").into_response(),
    }
}