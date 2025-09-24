mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;
mod routes;
mod helpers;

use sqlx::postgres::PgPoolOptions;
use axum::Router;
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL no está definida en .env");
    let app_port = env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Error conectando a la BD");

    let app = Router::new()
        .merge(routes::hospital::routes_hospital(pool.clone()))
        .merge(routes::patient::routes_patient(pool.clone()))
        .merge(routes::user::routes_user(pool.clone()))
        .merge(routes::doctor::routes_doctor(pool.clone()))
        .merge(routes::services::routes_services(pool.clone()))
        .merge(routes::speciality::routes_speciality(pool.clone()));

    let addr = format!("0.0.0.0:{}", app_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Servidor iniciado en http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
