use axum::{
    routing::{get},
    Router,
};
use std::sync::Arc;

use crate::{
    application::medical_history_service::MedicalHistoryService,
    infrastructure::medical_history_repository::PgMedicalHistoryRepository,
    interfaces::medical_history_controller,
};
use sqlx::PgPool;

pub fn routes_medical_history(pool: PgPool) -> Router {
    // 1. Crear el repositorio
    let repo = PgMedicalHistoryRepository::new(pool);

    // 2. Crear el servicio
    let service = Arc::new(MedicalHistoryService::new(repo));

    // 3. Construir el router con endpoints
    Router::new()
        .route(
            "/medical_history",
            get(medical_history_controller::get_all)
                .post(medical_history_controller::create),
        )
        .route(
            "/medical_history/{id}",
            get(medical_history_controller::get_by_id)
                .patch(medical_history_controller::update)
                .delete(medical_history_controller::delete),
        )
        .with_state(service)    

}   