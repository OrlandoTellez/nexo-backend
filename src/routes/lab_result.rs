use axum::{
    routing::{get},
    Router,
};
use std::sync::Arc;

use crate::{
    application::lab_result_service::LabResultService,
    infrastructure::lab_result::PgLabResultRepository,
    interfaces::lab_result_controller,
};
use sqlx::PgPool;

pub fn routes_lab_result(pool: PgPool) -> Router {
    // 1. Crear el repositorio
    let repo = PgLabResultRepository::new(pool);

    // 2. Crear el servicio
    let service = Arc::new(LabResultService::new(repo));

    // 3. Construir el router con endpoints
    Router::new()
        .route(
            "/lab_result",
            get(lab_result_controller::get_all)
                .post(lab_result_controller::create),
        )
        .route(
            "/lab_result/{id}",
            get(lab_result_controller::get_by_id)
                .patch(lab_result_controller::update)
                .delete(lab_result_controller::delete),
        )
        .with_state(service)    

}