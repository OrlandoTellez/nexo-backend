use axum::{
    routing::{get},
    Router,
};
use std::sync::Arc;

use crate::{
    application::doctor_service::DoctorService,
    infrastructure::doctor_repository::PgDoctorRepository,
    interfaces::doctor_controller,
};
use sqlx::PgPool;

pub fn routes_doctor(pool: PgPool) -> Router {
    // 1. Crear el repositorio
    let repo = PgDoctorRepository::new(pool);

    // 2. Crear el servicio
    let service = Arc::new(DoctorService::new(repo));

    // 3. Construir el router con endpoints
    Router::new()
        .route(
            "/doctors",
            get(doctor_controller::get_all)
                .post(doctor_controller::create),
        )
        .route(
            "/doctors/{id}",
            get(doctor_controller::get_by_id)
                .patch(doctor_controller::update)
                .delete(doctor_controller::delete),
        )
        .with_state(service)    

}