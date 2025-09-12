mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;

use application::hospital_service::HospitalService;
use axum::{routing::get, Router};
use infrastructure::hospital_repository::PgHospitalRepository;
use interfaces::hospital_controller;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .connect("postgresql://postgres:espadaLINUX@localhost:5432/db_paciente_app")
        .await
        .expect("Error conectando a la BD");

    let repo = PgHospitalRepository::new(pool);
    let service = Arc::new(HospitalService::new(repo));

    let app = Router::new()
        .route(
            "/hospitales",
            get(hospital_controller::get_all).post(hospital_controller::create),
        )
        .route(
            "/hospitales/{id}",
            get(hospital_controller::get_by_id)
                .put(hospital_controller::update)
                .delete(hospital_controller::delete),
        )
        .with_state(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
