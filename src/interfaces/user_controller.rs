use crate::application::user_service::UserService;
use crate::domain::user::{CreateUser, UpdateUser};
use crate::infrastructure::user_repository::PgUserRepository;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;

pub type SharedUserService = Arc<UserService<PgUserRepository>>;

pub async fn get_all(State(service): State<SharedUserService>) -> impl IntoResponse {
    match service.get_all().await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => {
            eprintln!("Error al obtener usuarios: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response()
        }
    }
}

pub async fn get_by_id(
    Path(id): Path<i32>,
    State(service): State<SharedUserService>,
) -> impl IntoResponse {
    match service.get_by_id(id).await {
        Ok(Some(u)) => (StatusCode::OK, Json(u)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Usuario no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al buscar usuario").into_response(),
    }
}

pub async fn create(
    State(service): State<SharedUserService>,
    Json(mut data): Json<CreateUser>,
) -> impl IntoResponse {
    // Hashear la contraseña
    match hash(&data.password_hash, DEFAULT_COST) {
        Ok(hashed) => data.password_hash = hashed,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error al hashear la contraseña",
            )
                .into_response()
        }
    }

    // Crear usuario con la contraseña hasheada
    match service.create(data).await {
        Ok(u) => (StatusCode::CREATED, Json(u)).into_response(),
        Err(e) => {
            eprintln!("Error al crear usuario: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al crear usuario").into_response()
        }
    }
}

pub async fn update(
    Path(id): Path<i32>,
    State(service): State<SharedUserService>,
    Json(mut data): Json<UpdateUser>,
) -> impl IntoResponse {
     // Si viene una contraseña, la hasheamos
    if let Some(ref password) = data.password_hash {
        match hash(password, DEFAULT_COST) {
            Ok(hashed) => data.password_hash = Some(hashed),
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error al hashear la contraseña",
                )
                    .into_response()
            }
        }
    }
    // Actualizar usuario 
    match service.update(id, data).await {
        Ok(Some(u)) => (StatusCode::OK, Json(u)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Usuario no encontrado").into_response(),
        Err(e) => {
            eprintln!("Error al actualizar usuario: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error al actualizar usuario",
            )
                .into_response()
        }
    }
}

pub async fn delete(
    Path(id): Path<i32>,
    State(service): State<SharedUserService>,
) -> impl IntoResponse {
    match service.delete(id).await {
        Ok(Some(u)) => (StatusCode::OK, Json(u)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Usuario no encontrado").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al eliminar usuario",
        )
            .into_response(),
    }
}
