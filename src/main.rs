mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;
mod routes;
mod helpers;

use sqlx::postgres::PgPoolOptions;
use axum::{Router, http};
use tower_http::cors::{CorsLayer};
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let frontend_url = env::var("FRONTEND_URL")
        .expect("FRONTEND_URL no está definida en .env");

    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<http::HeaderValue>().unwrap()) 
        .allow_methods([http::Method::GET, http::Method::POST, http::Method::PUT, http::Method::DELETE])
        .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
        .allow_credentials(true); 

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
        .merge(routes::speciality::routes_speciality(pool.clone()))
        .merge(routes::appointment::routes_appointment(pool.clone()))
        .merge(routes::medical_history::routes_medical_history(pool.clone()))
        .merge(routes::lab_result::routes_lab_result(pool.clone()))
        .merge(routes::auth::routes_auth(pool.clone()))
        .layer(cors)
        ;


    let addr = format!("0.0.0.0:{}", app_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Servidor iniciado en http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
