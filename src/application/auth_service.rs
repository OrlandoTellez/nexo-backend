use crate::{
    helpers::jwt::{generate_jwt, validate_jwt},
    infrastructure::auth_repository::AuthRepository,
};
use crate::domain::user::UserInfo;
use sqlx::PgPool;
use anyhow::Result;

pub struct AuthService<'a> {
    repo: AuthRepository<'a>,
}

impl<'a> AuthService<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self {
            repo: AuthRepository::new(pool),
        }
    }

    /// Intenta loguear y devolver un JWT
    pub async fn login(&self, username: &str, password: &str) -> Result<Option<(String, UserInfo)>> {
        if let Some(user_info) = self.repo.validate_user(username, password).await? {
            let token = generate_jwt(user_info.username.clone(), user_info.role.clone())?;
            Ok(Some((token, user_info)))
        } else {
            Ok(None)
        }
    }

    /// Verifica un JWT y devuelve claims
    pub fn verify_token(&self, token: &str) -> Result<String> {
        let data = validate_jwt(token)?;
        Ok(data.claims.sub)
    }
}
