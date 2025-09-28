use crate::domain::user::UserInfo;
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
        username: &str,
        password: &str,
    ) -> anyhow::Result<Option<UserInfo>> {
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
            if !is_valid {
                return Ok(None);
            }

            // Obtener datos adicionales según rol
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
        } else {
            Ok(None)
        }
    }
}
