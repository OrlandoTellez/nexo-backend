use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;

use crate::domain::patient::{Patient, CreatePatient, UpdatePatient};

#[async_trait]
pub trait PatientRepository: Send + Sync + 'static {
    async fn get_all(&self) -> Result<Vec<Patient>>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Patient>>;
    async fn create(&self, data: CreatePatient) -> Result<Patient>;
    async fn update(&self, id: i32, data: UpdatePatient) -> Result<Option<Patient>>;
    async fn delete(&self, id: i32) -> Result<Option<Patient>>;
}

pub struct PgPatientRepository {
    pool: PgPool,
}

impl PgPatientRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool}
    }
}

#[async_trait]
impl PatientRepository for PgPatientRepository {
    async fn get_all(&self) -> Result<Vec<Patient>> {
        let result: Vec<Patient> = sqlx::query_as::<_, Patient>(
            "SELECT id_patient, first_name, second_name, first_lastname, second_lastname, birthdate, phone, address, email FROM patients"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Patient>> {
        let result: Option<Patient> = sqlx::query_as::<_, Patient>(
            "SELECT id_patient, first_name, second_name, first_lastname, second_lastname, birthdate, phone, address, email FROM patients WHERE id_patient = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn create(&self, data: CreatePatient) -> Result<Patient> {
        let result: Patient = sqlx::query_as::<_, Patient>(
            "INSERT INTO patients (first_name, second_name, first_lastname, second_lastname, birthdate, phone, address, email) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(data.first_name)
        .bind(data.second_name)
        .bind(data.first_lastname)
        .bind(data.second_lastname)
        .bind(data.birthdate)
        .bind(data.phone)
        .bind(data.address)
        .bind(data.email)
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }

    async fn update(&self, id: i32, data: UpdatePatient) -> Result<Option<Patient>> {
        let result: Option<Patient> = sqlx::query_as::<_, Patient>(
            "UPDATE patients SET first_name = $1, second_name = $2, first_lastname = $3, second_lastname = $4, birthdate = $5, phone = $6, address = $7, email = $8 WHERE id_patient = $9"
        )
        .bind(data.first_name)
        .bind(data.second_name)
        .bind(data.first_lastname)
        .bind(data.second_lastname)
        .bind(data.birthdate)
        .bind(data.phone)
        .bind(data.address)
        .bind(data.email)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<Option<Patient>> {
        let result: Option<Patient> = sqlx::query_as::<_, Patient>(
            "DELETE FROM patients WHERE id_patient = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }
}


