use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::Error;

use crate::domain::services::{Service, CreateService, UpdateService};

#[async_trait]
pub trait ServiceRepository: Send + Sync + 'static {   
    async fn get_all(&self) -> Result<Vec<Service>>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Service>>;
    async fn create(&self, data: CreateService) -> Result<Service>;
    async fn update(&self, id: i32, data: UpdateService) -> Result<Option<Service>>;
    async fn delete(&self, id: i32) -> Result<Option<Service>>;
}

pub struct PgServiceRepository {
    pool: PgPool,
}

impl PgServiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ServiceRepository for PgServiceRepository { 
    async fn get_all(&self) -> Result<Vec<Service>> {
        let result: Vec<Service> = sqlx::query_as::<_, Service>(
            "SELECT id_service, service_name, created_at, updated_at, deleted_at FROM services WHERE deleted_at IS NULL"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Service>> {
        let result: Option<Service> = sqlx::query_as::<_, Service>(
            "SELECT id_service, service_name, created_at, updated_at, deleted_at FROM services WHERE id_service = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn create(&self, data: CreateService) -> Result<Service> {
        let query = sqlx::query_as::<_, Service>(
            "INSERT INTO services 
            (service_name) 
            VALUES ($1) 
            RETURNING *"
        )
        .bind(data.service_name);

        let result = match query.fetch_one(&self.pool).await {
            Ok(service) => service,
            Err(Error::Database(db_err)) => {
                if db_err.code().as_deref() == Some("23505") {
                    anyhow::bail!("El servicio ya existe")
                } else {
                    anyhow::bail!("Error en la base de datos: {}", db_err.message())
                }
            }
            Err(e) => {
                anyhow::bail!("Error inesperado: {}", e)
            }
        };

        Ok(result)
    }

    async fn update(&self, id: i32, data: UpdateService) -> Result<Option<Service>> {
        let result: Option<Service> = sqlx::query_as::<_, Service>(
            "UPDATE services SET
 service_name = COALESCE($1, service_name),
 updated_at = NOW()
 WHERE id_service = $2 RETURNING *",
        )
        .bind(data.service_name.as_deref())
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<Option<Service>> {
        let result: Option<Service> = sqlx::query_as::<_, Service>(
            "UPDATE services SET deleted_at = NOW() WHERE id_service = $1 RETURNING *",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }   
}