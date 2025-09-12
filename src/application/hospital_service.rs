use crate::domain::hospital::{CreateHospital, Hospital};
use crate::infrastructure::hospital_repository::HospitalRepository;
use anyhow::Result;

pub struct HospitalService<R: HospitalRepository> {
    repo: R,
}

impl<R: HospitalRepository> HospitalService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Hospital>> {
        self.repo.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Hospital>> {
        self.repo.get_by_id(id).await
    }

     pub async fn create(&self, data: CreateHospital) -> Result<Hospital> {
        self.repo.create(data).await
    }

    pub async fn update(&self, id: i32, data: CreateHospital) -> Result<Option<Hospital>> {
        self.repo.update(id, data).await
    }

    pub async fn delete(&self, id: i32) -> Result<Option<Hospital>> {
        self.repo.delete(id).await
    }

}
