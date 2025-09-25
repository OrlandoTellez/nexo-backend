use crate::domain::appointment::{Appointment, CreateAppointment, UpdateAppointment};
use crate::infrastructure::appointment_repository::AppointmentRepository;
use anyhow::Result;

pub struct AppointmentService<R: AppointmentRepository> {
    repo: R,
}

impl<R: AppointmentRepository> AppointmentService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Appointment>> {
        self.repo.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Appointment>> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, data: CreateAppointment) -> Result<Appointment> {
        self.repo.create(data).await
    }

    pub async fn update(&self, id: i32, data: UpdateAppointment) -> Result<Option<Appointment>> {        
        self.repo.update(id, data).await
    }

    pub async fn delete(&self, id: i32) -> Result<Option<Appointment>> {
        self.repo.delete(id).await
    }
}