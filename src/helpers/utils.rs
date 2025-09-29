use rand::Rng;

pub fn generate_patient_password(first_name: &str, first_lastname: &str, birth_year: i32) -> String {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(10..99); // dos dígitos aleatorios
    format!(
        "{}{}{}!{}",
        &first_name[0..1].to_uppercase(),
        first_lastname,
        birth_year,
        random_number
    )
}