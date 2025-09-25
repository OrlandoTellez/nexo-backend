use crate::domain::medical_history::{MedicalHistory, CreateMedicalHistory, UpdateMedicalHistory};
use crate::infrastructure::medical_history_repository::MedicalHistoryRepository;
use anyhow::Result;

pub struct MedicalHistoryService<R: MedicalHistoryRepository> {
    repo: R,
}

impl<R: MedicalHistoryRepository> MedicalHistoryService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<MedicalHistory>> {
        self.repo.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<MedicalHistory>> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, data: CreateMedicalHistory) -> Result<MedicalHistory> {
        self.repo.create(data).await
    }

    pub async fn update(&self, id: i32, data: UpdateMedicalHistory) -> Result<Option<MedicalHistory>> {        
        self.repo.update(id, data).await
    }

    pub async fn delete(&self, id: i32) -> Result<Option<MedicalHistory>> {
        self.repo.delete(id).await
    }
}