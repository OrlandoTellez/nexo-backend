use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::Error;

use crate::domain::user::{User, CreateUser, UpdateUser};

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn get_all(&self) -> Result<Vec<User>>;
    async fn get_by_id(&self, id: i32) -> Result<Option<User>>;
    async fn create(&self, data: CreateUser) -> Result<User>;
    async fn update(&self, id: i32, data: UpdateUser) -> Result<Option<User>>;
    async fn delete(&self, id: i32) -> Result<Option<User>>;
}

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn get_all(&self) -> Result<Vec<User>>{
        let result: Vec<User> = sqlx::query_as::<_, User>(
            "SELECT id_user, username, password_hash, role, created_at, updated_at, deleted_at FROM users WHERE deleted_at IS NULL"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<User>> {
        let result: Option<User> = sqlx::query_as::<_, User>(
            "SELECT id_user, username, password_hash, role, created_at, updated_at, deleted_at FROM users WHERE id_user = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn create(&self, data: CreateUser) -> Result<User> {
        let query = sqlx::query_as::<_, User>(
            "INSERT INTO users 
            (username, password_hash, role) 
            VALUES ($1,$2,$3) 
            RETURNING *"
        )
        .bind(data.username)
        .bind(data.password_hash)
        .bind(data.role);

        let result = match query.fetch_one(&self.pool).await {
            Ok(user) => user,
            Err(Error::Database(db_err)) => {
                if db_err.code().as_deref() == Some("23505") {
                    anyhow::bail!("El usuario ya existe")
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

    async fn update(&self, id: i32, data: UpdateUser) -> Result<Option<User>> {
        let result: Option<User> = sqlx::query_as::<_, User>(
            "UPDATE users SET
 username = COALESCE($1, username),
 password_hash = COALESCE($2, password_hash),
 role = COALESCE($3, role),
 updated_at = NOW()
 WHERE id_user = $4 RETURNING *",
        )
        .bind(data.username.as_deref())
        .bind(data.password_hash.as_deref())
        .bind(data.role.as_deref())
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<Option<User>> {
        let result: Option<User> = sqlx::query_as::<_, User>(
            "UPDATE users SET deleted_at = NOW() WHERE id_user = $1 RETURNING *",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }   
}