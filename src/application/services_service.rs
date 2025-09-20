use crate::domain::services::{Service, CreateService, UpdateService};
use crate::infrastructure::services_repository::ServiceRepository;
use anyhow::Result;

pub struct ServicesService<R: ServiceRepository> {
    repo: R,
}

impl<R: ServiceRepository> ServicesService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Service>> {
        self.repo.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Service>> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, data: CreateService) -> Result<Service> {
        self.repo.create(data).await
    }

    pub async fn update(&self, id: i32, data: UpdateService) -> Result<Option<Service>> {
        self.repo.update(id, data).await
    }

    pub async fn delete(&self, id: i32) -> Result<Option<Service>> {
        self.repo.delete(id).await
    }
}