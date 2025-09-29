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
            r#"
            SELECT id_patient, id_user, identity_number,
                   first_name, second_name, first_lastname, second_lastname,
                   gender, birthdate, blood_type,
                   phone, email, address,
                   emergency_contact_name, emergency_contact_phone,
                   allergies, current_medications, medical_background,
                   priority, status,
                   created_at, updated_at, deleted_at
            FROM patients
            WHERE deleted_at IS NULL
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Patient>> {
        let result: Option<Patient> = sqlx::query_as::<_, Patient>(
            r#"
            SELECT id_patient, id_user, identity_number,
                   first_name, second_name, first_lastname, second_lastname,
                   gender, birthdate, blood_type,
                   phone, email, address,
                   emergency_contact_name, emergency_contact_phone,
                   allergies, current_medications, medical_background,
                   priority, status,
                   created_at, updated_at, deleted_at
            FROM patients
            WHERE id_patient = $1 AND deleted_at IS NULL
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn create(&self, data: CreatePatient) -> Result<Patient> {
        let query = sqlx::query_as::<_, Patient>(
            r#"
            INSERT INTO patients (
                id_user, identity_number,
                first_name, second_name, first_lastname, second_lastname,
                gender, birthdate, blood_type,
                phone, email, address,
                emergency_contact_name, emergency_contact_phone,
                allergies, current_medications, medical_background,
                priority, status
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19)
            RETURNING *
            "#
        )
        .bind(data.id_user)
        .bind(data.identity_number)
        .bind(data.first_name)
        .bind(data.second_name)
        .bind(data.first_lastname)
        .bind(data.second_lastname)
        .bind(data.gender)
        .bind(data.birthdate)
        .bind(data.blood_type)
        .bind(data.phone)
        .bind(data.email)
        .bind(data.address)
        .bind(data.emergency_contact_name)
        .bind(data.emergency_contact_phone)
        .bind(data.allergies)
        .bind(data.current_medications)
        .bind(data.medical_background)
        .bind(data.priority.unwrap_or(0)) // valor por defecto
        .bind(data.status.unwrap_or_else(|| "active".to_string())); // valor por defecto

        let result = match query.fetch_one(&self.pool).await {
            Ok(patient) => patient,
            Err(Error::Database(db_err)) => {
                if db_err.code().as_deref() == Some("23505") {
                    anyhow::bail!("El número de cédula o correo ya existe")
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
            r#"
            UPDATE patients SET
                id_user = COALESCE($1, id_user),
                identity_number = COALESCE($2, identity_number),
                first_name = COALESCE($3, first_name),
                second_name = COALESCE($4, second_name),
                first_lastname = COALESCE($5, first_lastname),
                second_lastname = COALESCE($6, second_lastname),
                gender = COALESCE($7, gender),
                birthdate = COALESCE($8, birthdate),
                blood_type = COALESCE($9, blood_type),
                phone = COALESCE($10, phone),
                email = COALESCE($11, email),
                address = COALESCE($12, address),
                emergency_contact_name = COALESCE($13, emergency_contact_name),
                emergency_contact_phone = COALESCE($14, emergency_contact_phone),
                allergies = COALESCE($15, allergies),
                current_medications = COALESCE($16, current_medications),
                medical_background = COALESCE($17, medical_background),
                priority = COALESCE($18, priority),
                status = COALESCE($19, status),
                updated_at = NOW()
            WHERE id_patient = $20
            RETURNING *
            "#
        )
        .bind(data.id_user)
        .bind(data.identity_number)
        .bind(data.first_name)
        .bind(data.second_name)
        .bind(data.first_lastname)
        .bind(data.second_lastname)
        .bind(data.gender)
        .bind(data.birthdate)
        .bind(data.blood_type)
        .bind(data.phone)
        .bind(data.email)
        .bind(data.address)
        .bind(data.emergency_contact_name)
        .bind(data.emergency_contact_phone)
        .bind(data.allergies)
        .bind(data.current_medications)
        .bind(data.medical_background)
        .bind(data.priority)
        .bind(data.status)
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
