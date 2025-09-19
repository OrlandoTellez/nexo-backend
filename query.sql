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
    status VARCHAR(20) DEFAULT 'pending', -- pending, confirmed, completed, canceled
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

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
