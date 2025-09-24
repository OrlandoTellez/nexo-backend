use anyhow::Result;
use async_trait::async_trait;
use sqlx::Error;
use sqlx::PgPool;

use crate::domain::patient::{CreatePatient, Patient, UpdatePatient};

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
        Self { pool }
    }
}

#[async_trait]
impl PatientRepository for PgPatientRepository {
    async fn get_all(&self) -> Result<Vec<Patient>> {
        let result: Vec<Patient> = sqlx::query_as::<_, Patient>(
            " SELECT id_patient, id_user, first_name, second_name, first_lastname, second_lastname, 
            address, birthdate, phone, email, created_at, updated_at, deleted_at
            FROM patients
            WHERE deleted_at IS NULL"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Patient>> {
        let result: Option<Patient> = sqlx::query_as::<_, Patient>(
            "SELECT id_patient, first_name, second_name, first_lastname, second_lastname, birthdate, phone, address, email FROM patients WHERE id_patient = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn create(&self, data: CreatePatient) -> Result<Patient, anyhow::Error> {
        let query = sqlx::query_as::<_, Patient>(
        "INSERT INTO patients 
        (id_user, first_name, second_name, first_lastname, second_lastname, birthdate, phone, address, email) 
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) 
        RETURNING *"
    )
    .bind(data.id_user)
    .bind(data.first_name)
    .bind(data.second_name)
    .bind(data.first_lastname)
    .bind(data.second_lastname)
    .bind(data.birthdate)
    .bind(data.phone)
    .bind(data.address)
    .bind(data.email);

        let result = match query.fetch_one(&self.pool).await {
            Ok(patient) => patient,
            Err(Error::Database(db_err)) => {
                if db_err.code().as_deref() == Some("23505") {
                    anyhow::bail!("El correo ya existe")
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

    async fn update(&self, id: i32, data: UpdatePatient) -> Result<Option<Patient>> {
        let result: Option<Patient> = sqlx::query_as::<_, Patient>(
            "UPDATE patients SET 
            id_user = $1,
            first_name = $2,
            second_name = $3,
            first_lastname = $4,
            second_lastname = $5,
            birthdate = $6,
            phone = $7,
            address = $8,
            email = $9,
            updated_at = NOW()
         WHERE id_patient = $10
         RETURNING *",
        )
        .bind(data.id_user)
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
            "UPDATE patients SET deleted_at = NOW() WHERE id_patient = $1 RETURNING *",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }
}
