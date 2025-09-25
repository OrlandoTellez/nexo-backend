use axum::{
    routing::{get},
    Router,
};
use std::sync::Arc;

use crate::{
    application::appointment_service::AppointmentService,
    infrastructure::appointment_repository::PgAppointmentRepository,
    interfaces::appointment_controller,
};
use sqlx::PgPool;

pub fn routes_appointment(pool: PgPool) -> Router {
    // 1. Crear el repositorio
    let repo = PgAppointmentRepository::new(pool);

    // 2. Crear el servicio
    let service = Arc::new(AppointmentService::new(repo));

    // 3. Construir el router con endpoints
    Router::new()
        .route(
            "/appointments",
            get(appointment_controller::get_all)
                .post(appointment_controller::create),
        )
        .route(
            "/appointments/{id}",
            get(appointment_controller::get_by_id)
                .patch(appointment_controller::update)
                .delete(appointment_controller::delete),
        )
        .with_state(service)    

}