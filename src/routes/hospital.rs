use axum::{
    routing::{get},
    Router,
};
use std::sync::Arc;

use crate::{
    application::hospital_service::HospitalService,
    infrastructure::hospital_repository::PgHospitalRepository,
    interfaces::hospital_controller,
};
use sqlx::PgPool;

pub fn routes_hospital(pool: PgPool) -> Router {
    // 1. Crear el repositorio
    let repo = PgHospitalRepository::new(pool);

    // 2. Crear el servicio
    let service = Arc::new(HospitalService::new(repo));

    // 3. Construir el router con endpoints
    Router::new()
        .route(
            "/hospitales",
            get(hospital_controller::get_all)
                .post(hospital_controller::create),
        )
        .route(
            "/hospitales/{id}",
            get(hospital_controller::get_by_id)
                .put(hospital_controller::update)
                .delete(hospital_controller::delete),
        )
        .with_state(service) 
}
