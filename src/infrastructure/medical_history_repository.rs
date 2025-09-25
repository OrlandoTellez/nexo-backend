use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;

use crate::domain::medical_history::{MedicalHistory, CreateMedicalHistory, UpdateMedicalHistory};

#[async_trait]
pub trait MedicalHistoryRepository: Send + Sync + 'static {
    async fn get_all(&self) -> Result<Vec<MedicalHistory>>;
    async fn get_by_id(&self, id: i32) -> Result<Option<MedicalHistory>>;
    async fn create(&self, data: CreateMedicalHistory) -> Result<MedicalHistory>;
    async fn update(&self, id: i32, data: UpdateMedicalHistory) -> Result<Option<MedicalHistory>>;
    async fn delete(&self, id: i32) -> Result<Option<MedicalHistory>>;
}

pub struct PgMedicalHistoryRepository {
    pool: PgPool,
}

impl PgMedicalHistoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MedicalHistoryRepository for PgMedicalHistoryRepository {
    async fn get_all(&self) -> Result<Vec<MedicalHistory>> {
        let result: Vec<MedicalHistory> = sqlx::query_as::<_, MedicalHistory>(
            "SELECT * FROM medical_history WHERE deleted_at IS NULL"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<MedicalHistory>> {
        let result = sqlx::query_as::<_, MedicalHistory>(
            "SELECT * FROM medical_history WHERE id_history = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn create(&self, data: CreateMedicalHistory) -> Result<MedicalHistory> {
        let result = sqlx::query_as::<_, MedicalHistory>(
            "INSERT INTO medical_history 
            (id_patient, id_doctor, diagnosis, treatment, notes) 
            VALUES ($1,$2,$3,$4,$5) 
            RETURNING *"
        )
        .bind(data.id_patient)
        .bind(data.id_doctor)
        .bind(data.diagnosis)
        .bind(data.treatment)
        .bind(data.notes)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update(&self, id: i32, data: UpdateMedicalHistory) -> Result<Option<MedicalHistory>> {
        let result = sqlx::query_as::<_, MedicalHistory>(
            "UPDATE medical_history SET 
                diagnosis = COALESCE($1, diagnosis),
                treatment = COALESCE($2, treatment),
                notes = COALESCE($3, notes),
                updated_at = NOW()
             WHERE id_history = $4 AND deleted_at IS NULL
             RETURNING *"
        )
        .bind(data.diagnosis)
        .bind(data.treatment)
        .bind(data.notes)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<Option<MedicalHistory>> {
        let result = sqlx::query_as::<_, MedicalHistory>(
            "UPDATE medical_history SET deleted_at = NOW() WHERE id_history = $1 RETURNING *"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }
}