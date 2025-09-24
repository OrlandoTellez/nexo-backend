use validator::{ValidationError};
use regex::Regex;


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

pub fn validate_role(role: &str) -> Result<(), ValidationError> {
    match role {
        "patient" | "doctor" | "admin" => Ok(()),
        _ => Err(ValidationError::new("role")),
    }
}
