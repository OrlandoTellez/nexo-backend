use crate::domain::patient::{Patient, CreatePatient, UpdatePatient};
use crate::infrastructure::patient_repository::PatientRepository;
use anyhow::Result;

pub struct PatientService<R: PatientRepository> {
    repo: R,
}

impl<R: PatientRepository> PatientService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Patient>> {
        self.repo.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Patient>> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, data: CreatePatient, raw_password: &str) -> Result<Patient> {
        self.repo.create(data, raw_password).await
    }

    pub async fn update(&self, id: i32, data: UpdatePatient) -> Result<Option<Patient>> {
        self.repo.update(id, data).await
    }

    pub async fn delete(&self, id: i32) -> Result<Option<Patient>> {
        self.repo.delete(id).await
    }
 }

