use axum::{
    routing::{get, patch},
    Router,
};
use std::sync::Arc;

use crate::{
    application::user_service::UserService,
    infrastructure::user_repository::PgUserRepository,
    interfaces::user_controller,
};
use sqlx::PgPool;

pub fn routes_user(pool: PgPool) -> Router {
    // 1. Crear el repositorio
    let repo = PgUserRepository::new(pool);

    // 2. Crear el servicio
    let service = Arc::new(UserService::new(repo));

    // 3. Construir el router con endpoints
    Router::new()
        .route(
            "/users",
            get(user_controller::get_all)
                .post(user_controller::create),
        )
        .route(
            "/users/{id}",
            get(user_controller::get_by_id)
                .patch(user_controller::update)
                .delete(user_controller::delete),
        )
        .with_state(service)    

}
