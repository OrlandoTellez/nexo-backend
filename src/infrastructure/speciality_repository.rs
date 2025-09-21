use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::Error;

use crate::domain::speciality::{Speciality, CreateSpeciality, UpdateSpeciality};

#[async_trait]
pub trait SpecialityRepository: Send + Sync+ 'static {
    async fn get_all(&self) -> Result<Vec<Speciality>>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Speciality>>;
    async fn create(&self, data: CreateSpeciality) -> Result<Speciality>;
    async fn update(&self, id: i32, data: UpdateSpeciality) -> Result<Option<Speciality>>;
    async fn delete(&self, id: i32) -> Result<Option<Speciality>>;
}

pub struct PgSpecialityRepository {
    pool: PgPool
}

impl PgSpecialityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    } 
}

#[async_trait]
impl SpecialityRepository for PgSpecialityRepository {
    async fn get_all(&self) -> Result<Vec<Speciality>> {
        let result: Vec<Speciality> = sqlx::query_as::<_, Speciality>(
            "SELECT id_speciality, speciality_name, created_at, updated_at, deleted_at FROM specialities WHERE deleted_at IS NULL"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Speciality>> {
        let result: Option<Speciality> = sqlx::query_as::<_, Speciality>(
            "SELECT id_speciality, speciality_name, created_at, updated_at, deleted_at FROM specialities WHERE id_speciality = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn create(&self, data: CreateSpeciality) -> Result<Speciality>{
        let query = sqlx::query_as::<_, Speciality>(
            "INSERT INTO specialities (speciality_name) VALUES ($1) RETURNING *"
        ) 
        .bind(data.speciality_name);

        let result = match query.fetch_one(&self.pool).await{
            Ok(speciality) => speciality,
            Err(Error::Database(db_err)) => {
                if db_err.code().as_deref() == Some("23505") {
                    anyhow::bail!("La especialidad ya existe")
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

    async fn update(&self, id: i32, data: UpdateSpeciality) -> Result<Option<Speciality>>{
        let result: Option<Speciality> = sqlx::query_as::<_, Speciality>(
            "UPDATE specialities SET
            speciality_name = COALESCE($1, speciality_name)
            updated_at = NOW()
            WHERE id_speciality = $2 RETURNING *"
        )
        .bind(data.speciality_name)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<Option<Speciality>> {
        let result: Option<Speciality>  = sqlx::query_as::<_, Speciality>(
            "UPDATE users SET deleted_at = NOW() WHERE id_speciality = $1 RETURNING *"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }
}