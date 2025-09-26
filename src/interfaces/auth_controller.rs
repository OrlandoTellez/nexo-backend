use axum::{
    extract::{State, Json},
    response::{IntoResponse, Response},
    http::{StatusCode, header},
};
use serde::Deserialize;
use crate::application::auth_service::AuthService;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    let service = AuthService::new(&pool);

    match service.login(&payload.username, &payload.password).await {
        Ok(Some(token)) => {
            let cookie = format!(
                "auth_token={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=3600",
                token
            );

            (
                StatusCode::OK,
                [(header::SET_COOKIE, cookie)],
                "Login exitoso",
            )
                .into_response()
        }
        Ok(None) => (StatusCode::UNAUTHORIZED, "Credenciales inválidas").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error interno").into_response(),
    }
}

pub async fn logout_handler() -> Response {
    let cookie = "auth_token=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0".to_string();

    (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        "Logout exitoso",
    )
        .into_response()
}
