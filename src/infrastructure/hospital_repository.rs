use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;

use crate::domain::hospital::{Hospital, CreateHospital};

#[async_trait]
pub trait HospitalRepository: Send + Sync + 'static {
    async fn get_all(&self) -> Result<Vec<Hospital>>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Hospital>>;
    async fn create(&self, data: CreateHospital) -> Result<Hospital>;
    async fn update(&self, id: i32, data: CreateHospital) -> Result<Option<Hospital>>;
    async fn delete(&self, id: i32) -> Result<Option<Hospital>>;
}

pub struct PgHospitalRepository {
    pool: PgPool,
}

impl PgHospitalRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HospitalRepository for PgHospitalRepository {
    async fn get_all(&self) -> Result<Vec<Hospital>> {
        let result = sqlx::query_as::<_, Hospital>(
            "SELECT id_hospital, name, address FROM hospitals"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Hospital>> {
        let result = sqlx::query_as::<_, Hospital>(
            "SELECT id_hospital, name, address FROM hospitals WHERE id_hospital = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn create(&self, data: CreateHospital) -> Result<Hospital> {
        let result = sqlx::query_as::<_, Hospital>(
            "INSERT INTO hospitals (name, address) VALUES ($1, $2) RETURNING id_hospital, name, address"
        )
        .bind(data.name)
        .bind(data.address)
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }

    async fn update(&self, id: i32, data: CreateHospital) -> Result<Option<Hospital>> {
        let result = sqlx::query_as::<_, Hospital>(
            "UPDATE hospitals SET name = $1, address = $2 WHERE id_hospital = -$3 RETURNING id_hospital, name, address"
        )
        .bind(data.name)
        .bind(data.address)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<Option<Hospital>> {
        let result = sqlx::query_as::<_, Hospital>(
            "DELETE FROM hospitals WHERE id_hospital = $1 RETURNING id_hospital, name, address"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }
}
