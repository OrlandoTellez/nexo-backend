use axum::{
    routing::{get},
    Router,
};
use std::sync::Arc;

use crate::{
    application::services_service::ServicesService,
    infrastructure::services_repository::PgServiceRepository,
    interfaces::services_controller,
};
use sqlx::PgPool;

pub fn routes_services(pool: PgPool) -> Router {
    // 1. Crear el repositorio
    let repo = PgServiceRepository::new(pool);

    // 2. Crear el servicio
    let service = Arc::new(ServicesService::new(repo));

    // 3. Construir el router con endpoints
    Router::new()
        .route(
            "/services",
            get(services_controller::get_all)
                .post(services_controller::create),
        )
        .route(
            "/services/{id}",
            get(services_controller::get_by_id)
                .patch(services_controller::update)
                .delete(services_controller::delete),
        )
        .with_state(service)    

}   