use sqlx::PgPool;
use bcrypt::verify;

pub struct AuthRepository<'a> {
    pub pool: &'a PgPool,
}

impl<'a> AuthRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Valida usuario y contraseña
    pub async fn validate_user(&self, username: &str, password: &str) -> anyhow::Result<Option<(i32, String)>> {
        let row = sqlx::query!(
            r#"
            SELECT id_user, username, password_hash, role
            FROM users
            WHERE username = $1 AND deleted_at IS NULL
            "#,
            username
        )
        .fetch_optional(self.pool)
        .await?;

        if let Some(user) = row {
            let is_valid = verify(password, &user.password_hash)?;
            if is_valid {
                Ok(Some((user.id_user, user.role)))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
