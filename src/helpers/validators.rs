use validator::{ValidationError};
use regex::Regex;

/// Validar que un teléfono cumpla con el formato E.164 (+50575061202)
pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    lazy_static::lazy_static! {
        static ref PHONE_REGEX: Regex = Regex::new(r"^\+[1-9][0-9]{7,14}$").unwrap();
    }

    if PHONE_REGEX.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationError::new("phone"))
    }
}
