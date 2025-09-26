-- Database: db_paciente_app

-- DROP DATABASE IF EXISTS db_paciente_app;

CREATE DATABASE db_paciente_app
    WITH
    OWNER = postgres
    ENCODING = 'UTF8'
    LC_COLLATE = 'en_US.UTF-8'
    LC_CTYPE = 'en_US.UTF-8'
    LOCALE_PROVIDER = 'libc'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1
    IS_TEMPLATE = False;

CREATE TABLE hospitals (
    id_hospital SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    address VARCHAR(200) NOT NULL
);


-- Tabla de Usuarios (Login y Roles)
CREATE TABLE users (
    id_user SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL, -- guarda hash, no texto plano
    role VARCHAR(20) NOT NULL,   -- 'patient', 'doctor', 'admin'
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Tabla de Áreas Médicas
CREATE TABLE areas (
    id_area SERIAL PRIMARY KEY,
    area_name VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

--Tabla de Especialidades
CREATE TABLE specialities (
    id_speciality SERIAL PRIMARY KEY,
    speciality_name VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Tabla de Servicios Médicos
CREATE TABLE services (
    id_service SERIAL PRIMARY KEY,
    service_name VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Tabla de Pacientes
CREATE TABLE patients (
    id_patient SERIAL PRIMARY KEY,
    id_user INT NULL REFERENCES users(id_user),
    first_name VARCHAR(50) NOT NULL,
    second_name VARCHAR(50),
    first_lastname VARCHAR(50) NOT NULL,
    second_lastname VARCHAR(50),
    address VARCHAR(300),
    birthdate DATE NOT NULL,
    phone VARCHAR(20),
    email VARCHAR(100) UNIQUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Tabla de Doctores
CREATE TABLE doctors (
    id_doctor SERIAL PRIMARY KEY,
    id_area INT NOT NULL REFERENCES areas(id_area),
	id_speciality INT NULL REFERENCES specialities(id_speciality),
    id_service INT NOT NULL REFERENCES services(id_service),
    id_user INT NULL REFERENCES users(id_user),
    first_name VARCHAR(50) NOT NULL,
    second_name VARCHAR(50),
    first_lastname VARCHAR(50) NOT NULL,
    second_lastname VARCHAR(50),
    phone VARCHAR(20),
    email VARCHAR(100) UNIQUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Tabla de Citas Médicas
CREATE TABLE medical_appointments (
    id_appointment SERIAL PRIMARY KEY,
    id_patient INT NOT NULL REFERENCES patients(id_patient),
    id_doctor INT NOT NULL REFERENCES doctors(id_doctor),
    id_area INT NOT NULL REFERENCES areas(id_area),
    id_service INT NOT NULL REFERENCES services(id_service),
    appointment_datetime TIMESTAMP NOT NULL,
    building VARCHAR(10),
    room VARCHAR(10),
    notes TEXT,
    prescription TEXT,
    status VARCHAR(20) DEFAULT 'pending', -- pending, confirmed, completed, canceled
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Historial clinico
CREATE TABLE medical_history (
    id_history SERIAL PRIMARY KEY,
    id_patient INT NOT NULL REFERENCES patients(id_patient),
    id_doctor INT REFERENCES doctors(id_doctor),
    diagnosis TEXT NOT NULL,
    treatment TEXT,
    notes TEXT,
    record_date TIMESTAMP DEFAULT NOW(),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);

-- Resultados de laboratorio
CREATE TABLE lab_results (
    id_result SERIAL PRIMARY KEY,
    id_patient INT NOT NULL REFERENCES patients(id_patient),
    id_doctor INT REFERENCES doctors(id_doctor),
    lab_name VARCHAR(100) NOT NULL,
    test_type VARCHAR(100),
    result TEXT NOT NULL,
    result_date TIMESTAMP DEFAULT NOW(),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);

-- Notificaciones
CREATE TABLE notifications (
    id_notification SERIAL PRIMARY KEY,
    id_user INT NOT NULL REFERENCES users(id_user),
    title VARCHAR(200) NOT NULL,
    message TEXT NOT NULL,
    type VARCHAR(50), -- 'cita', 'resultado', 'general'
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    read_at TIMESTAMP
);

-- Documentos médicos
CREATE TABLE medical_documents (
    id_document SERIAL PRIMARY KEY,
    id_patient INT NOT NULL REFERENCES patients(id_patient),
    id_doctor INT REFERENCES doctors(id_doctor),
    document_type VARCHAR(100), -- 'imagen', 'PDF', 'radiografía'
    file_path TEXT NOT NULL,    -- ruta en el servidor o URL
    description TEXT,
    uploaded_at TIMESTAMP DEFAULT NOW()
);


-- Tabla de turnos virtuales
CREATE TABLE virtual_turns (
  id_turn SERIAL PRIMARY KEY,
  id_patient INT NULL REFERENCES patients(id_patient) ON DELETE SET NULL,
  id_service INT NOT NULL REFERENCES services(id_service) ON DELETE CASCADE,
  id_area INT NULL REFERENCES areas(id_area) ON DELETE SET NULL,
  id_desk INT NULL REFERENCES users(id_user) ON DELETE SET NULL, -- operador/recepción que llamó
  turn_number INT NOT NULL,
  priority INT DEFAULT 0, -- 0 normal, mayor prioridad = antes
  status VARCHAR(20) NOT NULL DEFAULT 'waiting', 
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  called_at TIMESTAMP NULL,
  started_at TIMESTAMP NULL,
  completed_at TIMESTAMP NULL,
  expires_at TIMESTAMP NULL,
  metadata JSONB DEFAULT '{}'::jsonb,

  -- Columna generada: fecha del turno
  turn_date DATE GENERATED ALWAYS AS (created_at::date) STORED,

  -- Restricción de unicidad: un número por servicio y fecha
  CONSTRAINT unique_turn_number_per_service_per_day UNIQUE (id_service, turn_number, turn_date)
);

-- Índices
CREATE INDEX idx_turns_service_status ON virtual_turns (id_service, status, created_at);
CREATE INDEX idx_turns_patient ON virtual_turns (id_patient);
CREATE INDEX idx_turns_service_turn ON virtual_turns (id_service, turn_number);

-- Tokens de dispositivos (notificaciones push)
CREATE TABLE device_tokens (
  id_token SERIAL PRIMARY KEY,
  id_user INT NOT NULL REFERENCES users(id_user) ON DELETE CASCADE,
  token TEXT NOT NULL,
  platform VARCHAR(10) NOT NULL CHECK (platform IN ('fcm', 'apns')), -- firebase o apple
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  last_used TIMESTAMP NULL
);

CREATE UNIQUE INDEX idx_user_token_platform ON device_tokens (id_user, token, platform);

-- Audit log de cambios de estado de turnos
CREATE TABLE turn_audit_logs (
  id_log SERIAL PRIMARY KEY,
  id_turn INT NOT NULL REFERENCES virtual_turns(id_turn) ON DELETE CASCADE,
  id_user INT NULL REFERENCES users(id_user),
  old_status VARCHAR(20),
  new_status VARCHAR(20),
  changed_at TIMESTAMP NOT NULL DEFAULT NOW(),
  note TEXT
);

CREATE INDEX idx_turn_audit_turn ON turn_audit_logs (id_turn, changed_at);

-- Tabla de Auditoría para cambios críticos
CREATE TABLE audit_logs (
    id_audit SERIAL PRIMARY KEY,
    table_name VARCHAR(50) NOT NULL,       -- nombre de la tabla
    record_id INT NOT NULL,                -- id del registro modificado
    action VARCHAR(20) NOT NULL,           -- 'insert', 'update', 'delete'
    old_data JSONB,                        -- datos antes del cambio
    new_data JSONB,                        -- datos después del cambio
    performed_by INT REFERENCES users(id_user), -- quién hizo el cambio
    performed_at TIMESTAMP DEFAULT NOW()
);
