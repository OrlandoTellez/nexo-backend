use crate::domain::appointment::{Appointment, CreateAppointment, UpdateAppointment};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool};

#[async_trait]
pub trait AppointmentRepository: Send + Sync + 'static {
    async fn get_all(&self) -> Result<Vec<Appointment>>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Appointment>>;
    async fn create(&self, data: CreateAppointment) -> Result<Appointment>;
    async fn update(&self, id: i32, data: UpdateAppointment) -> Result<Option<Appointment>>;
    async fn delete(&self, id: i32) -> Result<Option<Appointment>>;
}

pub struct PgAppointmentRepository {
    pool: PgPool,
}

impl PgAppointmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AppointmentRepository for PgAppointmentRepository {
    async fn get_all(&self) -> Result<Vec<Appointment>> {
        let result: Vec<Appointment> = sqlx::query_as::<_, Appointment>(
            "SELECT * FROM medical_appointments WHERE deleted_at IS NULL"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Appointment>> {
        let result = sqlx::query_as::<_, Appointment>(
            "SELECT * FROM medical_appointments WHERE id_appointment = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn create(&self, data: CreateAppointment) -> Result<Appointment> {
        let result = sqlx::query_as::<_, Appointment>(
            "INSERT INTO medical_appointments
            (id_patient, id_doctor, id_area, id_service, appointment_datetime, building, room, notes)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
            RETURNING *"
        )
        .bind(data.id_patient)
        .bind(data.id_doctor)
        .bind(data.id_area)
        .bind(data.id_service)
        .bind(data.appointment_datetime)
        .bind(data.building)
        .bind(data.room)
        .bind(data.notes)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update(&self, id: i32, data: UpdateAppointment) -> Result<Option<Appointment>> {
        let result = sqlx::query_as::<_, Appointment>(
            "UPDATE medical_appointments SET 
                appointment_datetime = COALESCE($1, appointment_datetime),
                building = COALESCE($2, building),
                room = COALESCE($3, room),
                notes = COALESCE($4, notes),
                prescription = COALESCE($5, prescription),
                status = COALESCE($6, status),
                updated_at = NOW()
             WHERE id_appointment = $7 AND deleted_at IS NULL
             RETURNING *"
        )
        .bind(data.appointment_datetime)
        .bind(data.building)
        .bind(data.room)
        .bind(data.notes)
        .bind(data.prescription)
        .bind(data.status)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<Option<Appointment>> {
        let result = sqlx::query_as::<_, Appointment>(
            "UPDATE medical_appointments SET deleted_at = NOW() WHERE id_appointment = $1 RETURNING *"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }
}
