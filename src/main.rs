mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;
mod routes;

use sqlx::postgres::PgPoolOptions;
use axum::Router;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .connect("postgresql://postgres:espadaLINUX@localhost:5432/db_paciente_app")
        .await
        .expect("Error conectando a la BD");

    let app = Router::new()
        .merge(routes::hospital::routes_hospital(pool.clone()))
        .merge(routes::patient::routes_patient(pool.clone()))
        .merge(routes::user::routes_user(pool.clone()))
        .merge(routes::doctor::routes_doctor(pool.clone()))
        ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
