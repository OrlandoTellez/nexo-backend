use crate::domain::doctor::{Doctor, CreateDoctor, UpdateDoctor};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::Error;

#[async_trait]
pub trait DoctorRepository: Send + Sync + 'static {
    async fn get_all(&self) -> Result<Vec<Doctor>>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Doctor>>;
    async fn create(&self, data: CreateDoctor) -> Result<Doctor>;
    async fn update(&self, id: i32, data: UpdateDoctor) -> Result<Option<Doctor>>;
    async fn delete(&self, id: i32) -> Result<Option<Doctor>>;
}

pub struct PgDoctorRepository {
    pool: PgPool,
}

impl PgDoctorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DoctorRepository for PgDoctorRepository {
    async fn get_all(&self) -> Result<Vec<Doctor>> {
        let result: Vec<Doctor> = sqlx::query_as::<_, Doctor>(
            "SELECT id_doctor, id_area, id_speciality, id_service, id_user, first_name, second_name, first_lastname, second_lastname, phone, email, created_at, updated_at, deleted_at FROM doctors WHERE deleted_at IS NULL"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Doctor>> {
        let result: Option<Doctor> = sqlx::query_as::<_, Doctor>(
            "SELECT id_doctor, id_area, id_speciality, id_service, id_user, first_name, second_name, first_lastname, second_lastname, phone, email, created_at, updated_at, deleted_at FROM doctors WHERE id_doctor = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }                

    async fn create(&self, data: CreateDoctor) -> Result<Doctor> {
        let query = sqlx::query_as::<_, Doctor>(
            "INSERT INTO doctors 
            (id_area, id_service, id_speciality, id_user, first_name, second_name, first_lastname, second_lastname, phone, email) 
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) 
            RETURNING *"
        )
        .bind(data.id_area)
        .bind(data.id_service)
        .bind(data.id_speciality)
        .bind(data.id_user)
        .bind(data.first_name)
        .bind(data.second_name)
        .bind(data.first_lastname)
        .bind(data.second_lastname)
        .bind(data.phone)
        .bind(data.email);

        let result = match query.fetch_one(&self.pool).await {
            Ok(doctor) => doctor,
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

    async fn update(&self, id: i32, data: UpdateDoctor) -> Result<Option<Doctor>> {
        let result: Option<Doctor> = sqlx::query_as::<_, Doctor>(
            "UPDATE doctors SET 
            id_area = $1,
            id_service = $2,
            id_speciality = $3,
            id_user = $4,
            first_name = $5,
            second_name = $6,
            first_lastname = $7,
            second_lastname = $8,
            phone = $9,
            email = $10,
            updated_at = NOW()            
         WHERE id_doctor = $11
         RETURNING *",
        )
        .bind(data.id_area)
        .bind(data.id_service)
        .bind(data.id_speciality)
        .bind(data.id_user)
        .bind(data.first_name)
        .bind(data.second_name)
        .bind(data.first_lastname)
        .bind(data.second_lastname)
        .bind(data.phone)
        .bind(data.email)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<Option<Doctor>> {
        let result: Option<Doctor> = sqlx::query_as::<_, Doctor>(
            "UPDATE doctors SET deleted_at = NOW() WHERE id_doctor = $1 RETURNING *",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }
}   