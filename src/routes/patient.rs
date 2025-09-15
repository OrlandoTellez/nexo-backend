use axum::{
    routing::{get},
    Router,
};
use std::sync::Arc;

use crate::{
    application::patient_service::PatientService,
    infrastructure::patient_repository::PgPatientRepository,
    interfaces::patient_controller,
};
use sqlx::PgPool;

pub fn routes_patient(pool: PgPool) -> Router {
    // 1. Crear el repositorio
    let repo = PgPatientRepository::new(pool);

    // 2. Crear el servicio
    let service = Arc::new(PatientService::new(repo));

    // 3. Construir el router con endpoints
    Router::new()
        .route(
            "/pacientes",
            get(patient_controller::get_all)
                .post(patient_controller::create),
        )
        .route(
            "/patientes/{id}",
            get(patient_controller::get_by_id)
                .put(patient_controller::update)
                .delete(patient_controller::delete),
        )
        .with_state(service)    

}