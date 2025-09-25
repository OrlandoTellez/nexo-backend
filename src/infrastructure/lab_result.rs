use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;

use crate::domain::lab_result::{LabResult, CreateLabResult, UpdateLabResult};

#[async_trait]
pub trait LabResultRepository: Send + Sync + 'static {
    async fn get_all(&self) -> Result<Vec<LabResult>>;
    async fn get_by_id(&self, id: i32) -> Result<Option<LabResult>>;
    async fn create(&self, data: CreateLabResult) -> Result<LabResult>;
    async fn update(&self, id: i32, data: UpdateLabResult) -> Result<Option<LabResult>>;
    async fn delete(&self, id: i32) -> Result<Option<LabResult>>;
}

pub struct PgLabResultRepository {
    pool: PgPool,
}

impl PgLabResultRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LabResultRepository for PgLabResultRepository {
    async fn get_all(&self) -> Result<Vec<LabResult>> {
        let result: Vec<LabResult> = sqlx::query_as::<_, LabResult>(
            "SELECT * FROM lab_results WHERE deleted_at IS NULL"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<LabResult>> {
        let result = sqlx::query_as::<_, LabResult>(
            "SELECT * FROM lab_results WHERE id_result = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn create(&self, data: CreateLabResult) -> Result<LabResult> {
        let result = sqlx::query_as::<_, LabResult>(
            "INSERT INTO lab_results 
            (id_patient, id_doctor, lab_name, test_type, result) 
            VALUES ($1,$2,$3,$4,$5) 
            RETURNING *"
        )
        .bind(data.id_patient)
        .bind(data.id_doctor)
        .bind(data.lab_name)
        .bind(data.test_type)
        .bind(data.result)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update(&self, id: i32, data: UpdateLabResult) -> Result<Option<LabResult>> {
        let result = sqlx::query_as::<_, LabResult>(
            "UPDATE lab_results SET 
                lab_name = COALESCE($1, lab_name),
                test_type = COALESCE($2, test_type),
                result = COALESCE($3, result),
                updated_at = NOW()
             WHERE id_result = $4 AND deleted_at IS NULL
             RETURNING *"
        )
        .bind(data.lab_name)
        .bind(data.test_type)
        .bind(data.result)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<Option<LabResult>> {
        let result = sqlx::query_as::<_, LabResult>(
            "UPDATE lab_results SET deleted_at = NOW() WHERE id_result = $1 RETURNING *"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }
}