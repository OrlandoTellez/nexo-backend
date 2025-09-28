use crate::domain::user::{AuthUserRaw, UserInfo};
use bcrypt::verify;
use sqlx::PgPool;

pub struct AuthRepository<'a> {
    pub pool: &'a PgPool,
}

impl<'a> AuthRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

   pub async fn validate_user(
    &self,
    identifier: &str, // puede ser username o email
    password: &str,
) -> anyhow::Result<Option<UserInfo>> {
    // 1. Buscar primero por username
    let mut user = sqlx::query_as!(
        AuthUserRaw,
        r#"
        SELECT id_user, username, password_hash, role
        FROM users
        WHERE username = $1
          AND deleted_at IS NULL
        "#,
        identifier
    )
    .fetch_optional(self.pool)
    .await?;

    // 2. Si no se encontró, buscar por email en cada tabla según corresponda
    if user.is_none() {
        // Paciente
        user = sqlx::query_as!(
            AuthUserRaw,
            r#"
            SELECT u.id_user, u.username, u.password_hash, u.role
            FROM users u
            JOIN patients p ON p.id_user = u.id_user
            WHERE p.email = $1
              AND u.deleted_at IS NULL
            "#,
            identifier
        )
        .fetch_optional(self.pool)
        .await?;

        // Doctor
        if user.is_none() {
            user = sqlx::query_as!(
                AuthUserRaw,
                r#"
                SELECT u.id_user, u.username, u.password_hash, u.role
                FROM users u
                JOIN doctors d ON d.id_user = u.id_user
                WHERE d.email = $1
                  AND u.deleted_at IS NULL
                "#,
                identifier
            )
            .fetch_optional(self.pool)
            .await?;
        }

        // Admisionist
        if user.is_none() {
            user = sqlx::query_as!(
                AuthUserRaw,
                r#"
                SELECT u.id_user, u.username, u.password_hash, u.role
                FROM users u
                JOIN admisionists a ON a.id_user = u.id_user
                WHERE a.email = $1
                  AND u.deleted_at IS NULL
                "#,
                identifier
            )
            .fetch_optional(self.pool)
            .await?;
        }
    }

    // 3. Si no se encontró ningún usuario
    let user = match user {
        Some(u) => u,
        None => return Ok(None),
    };

    // 4. Verificar contraseña
    let is_valid = verify(password, &user.password_hash)?;
    if !is_valid {
        return Ok(None);
    }

    // 5. Obtener datos adicionales según el rol
    let user_info = match user.role.as_str() {
        "doctor" => {
            let doc = sqlx::query!(
                r#"
                SELECT first_name, first_lastname, email, phone
                FROM doctors
                WHERE id_user = $1
                "#,
                user.id_user
            )
            .fetch_optional(self.pool)
            .await?;

            UserInfo {
                id: user.id_user,
                username: user.username,
                role: user.role,
                first_name: doc.as_ref().map(|d| d.first_name.clone()),
                last_name: doc.as_ref().map(|d| d.first_lastname.clone()),
                email: doc.as_ref().and_then(|d| d.email.clone()),
                phone: doc.as_ref().and_then(|d| d.phone.clone()),
            }
        }
        "patient" => {
            let pat = sqlx::query!(
                r#"
                SELECT first_name, first_lastname, email, phone
                FROM patients
                WHERE id_user = $1
                "#,
                user.id_user
            )
            .fetch_optional(self.pool)
            .await?;

            UserInfo {
                id: user.id_user,
                username: user.username,
                role: user.role,
                first_name: pat.as_ref().map(|p| p.first_name.clone()),
                last_name: pat.as_ref().map(|p| p.first_lastname.clone()),
                email: pat.as_ref().and_then(|p| p.email.clone()),
                phone: pat.as_ref().and_then(|p| p.phone.clone()),
            }
        }
        "admisionist" => {
            let adm = sqlx::query!(
                r#"
                SELECT first_name, first_lastname, email, phone
                FROM admisionists
                WHERE id_user = $1
                "#,
                user.id_user
            )
            .fetch_optional(self.pool)
            .await?;

            UserInfo {
                id: user.id_user,
                username: user.username,
                role: user.role,
                first_name: adm.as_ref().map(|a| a.first_name.clone()),
                last_name: adm.as_ref().map(|a| a.first_lastname.clone()),
                email: adm.as_ref().and_then(|a| a.email.clone()),
                phone: adm.as_ref().and_then(|a| a.phone.clone()),
            }
        }
        "admin" => UserInfo {
            id: user.id_user,
            username: user.username,
            role: user.role,
            first_name: None,
            last_name: None,
            email: None,
            phone: None,
        },
        _ => return Ok(None),
    };

    Ok(Some(user_info))
}

}
