use axum::{Router, routing::post};
use sqlx::PgPool;
use crate::interfaces::auth_controller::{login_handler, logout_handler};

pub fn routes_auth(pool: PgPool) -> Router {
    return Router::new()
        .route("/auth/login", post(login_handler))
        .route("/auth/logout", post(logout_handler))
        .with_state(pool);
}