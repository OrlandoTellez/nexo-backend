use crate::domain::user::{User, CreateUser, UpdateUser};
use crate::infrastructure::user_repository::UserRepository;
use anyhow::Result;

pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<User>> {
        self.repo.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<User>> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, data: CreateUser) -> Result<User> {
        self.repo.create(data).await
    }

    pub async fn update(&self, id: i32, data: UpdateUser) -> Result<Option<User>> {
        self.repo.update(id, data).await
    }

    pub async fn delete(&self, id: i32) -> Result<Option<User>> {
        self.repo.delete(id).await
    }
}

