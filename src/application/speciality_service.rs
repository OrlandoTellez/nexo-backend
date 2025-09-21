use crate::domain::speciality::{Speciality, CreateSpeciality, UpdateSpeciality};
use crate::infrastructure::speciality_repository::SpecialityRepository;
use anyhow::Result;

pub struct SpecialityService<R: SpecialityRepository>{
    repo: R,
}

impl <R: SpecialityRepository> SpecialityService<R> {
    pub fn new(repo: R) -> Self{
        Self {repo}
    }

    pub async fn get_all(&self) -> Result<Vec<Speciality>> {
        self.repo.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Speciality>> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, data: CreateSpeciality) -> Result<Speciality> {
        self.repo.create(data).await
    }

    pub async fn update(&self, id: i32, data: UpdateSpeciality) -> Result<Option<Speciality>> {
        self.repo.update(id, data).await
    } 

    pub async fn delete(&self, id: i32) -> Result<Option<Speciality>> {
        self.repo.delete(id).await
    }
} 