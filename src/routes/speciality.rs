use axum::{
    routing::{get},
    Router,
};
use std::sync::Arc;

use crate::{
    application::speciality_service::SpecialityService,
    infrastructure::speciality_repository::PgSpecialityRepository,
    interfaces::speciality_controller,
};
use sqlx::PgPool;

pub fn routes_speciality(pool: PgPool) -> Router {
    // 1. Crear el repositorio
    let repo: PgSpecialityRepository = PgSpecialityRepository::new(pool);

    // 2. Crear el servicio
    let service = Arc::new(SpecialityService::new(repo));

    // 3. Construir el router con endpoints
    Router::new()
        .route(
            "/specialities",
            get(speciality_controller::get_all)
            .post(speciality_controller::create),
        )
        .route(
            "/specialities/{id}", 
            get(speciality_controller::get_by_id)
            .patch(speciality_controller::update)
            .delete(speciality_controller::delete)
        )
        .with_state(service)

}