use crate::domain::doctor::{Doctor, CreateDoctor, UpdateDoctor};
use crate::infrastructure::doctor_repository::DoctorRepository;
use anyhow::Result;

pub struct DoctorService<R: DoctorRepository> {
    repo: R,
}

impl<R: DoctorRepository> DoctorService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Doctor>> {
        self.repo.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Doctor>> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, data: CreateDoctor) -> Result<Doctor> {
        self.repo.create(data).await
    }

    pub async fn update(&self, id: i32, data: UpdateDoctor) -> Result<Option<Doctor>> {        
        self.repo.update(id, data).await
    }

    pub async fn delete(&self, id: i32) -> Result<Option<Doctor>> {
        self.repo.delete(id).await
    }
}