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

-- Tabla de hospitales
CREATE TABLE hospitals (
    id_hospital SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    address VARCHAR(200) NOT NULL
);

-- Tabla de pacientes
CREATE TABLE patients (
    id_patient SERIAL PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    second_name VARCHAR(50),
    first_lastname VARCHAR(50) NOT NULL,
    second_lastname VARCHAR(50),
    birthdate DATE NOT NULL,
    phone VARCHAR(15),
    address VARCHAR(300),
    email VARCHAR(100) UNIQUE,
    created_at TIMESTAMP DEFAULT NOW()
);
