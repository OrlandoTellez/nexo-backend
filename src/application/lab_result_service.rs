use crate::domain::lab_result::{LabResult, CreateLabResult, UpdateLabResult};
use crate::infrastructure::lab_result::LabResultRepository;
use anyhow::Result;

pub struct LabResultService<R: LabResultRepository> {
    repo: R,
}

impl<R: LabResultRepository> LabResultService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<LabResult>> {
        self.repo.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<LabResult>> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, data: CreateLabResult) -> Result<LabResult> {
        self.repo.create(data).await
    }

    pub async fn update(&self, id: i32, data: UpdateLabResult) -> Result<Option<LabResult>> {        
        self.repo.update(id, data).await
    }

    pub async fn delete(&self, id: i32) -> Result<Option<LabResult>> {
        self.repo.delete(id).await
    }
}