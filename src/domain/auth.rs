use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

/// Datos que se guardan en el JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // username o id_user
    pub role: String,  // "doctor", "patient", "admin"
    pub exp: usize,    // fecha de expiración (timestamp)
}

impl Claims {
    pub fn new(sub: String, role: String, exp_minutes: i64) -> Self {
        let exp = (Utc::now() + Duration::minutes(exp_minutes)).timestamp() as usize;
        Claims { sub, role, exp }
    }
}
