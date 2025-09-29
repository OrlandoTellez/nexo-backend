# Documentación Completa de la Base de Datos `db_paciente_app`

## Descripción General

La base de datos de `nexo` está diseñada para soportar un **sistema integral de gestión hospitalaria**, permitiendo almacenar y gestionar información completa de pacientes, doctores, citas médicas, historial clínico, resultados de laboratorio, y todos los procesos administrativos de un centro de salud.

### Características Principales

- **Arquitectura Relacional Completa**: Diseño normalizado con relaciones bien definidas
- **Sistema de Auditoría Integral**: Tracking completo de cambios y acciones
- **Soft Delete**: Eliminación lógica para mantener integridad de datos históricos
- **Validación de Datos**: Constraints a nivel de base de datos
- **Índices Optimizados**: Para consultas de alto rendimiento
- **Soporte para JSONB**: Datos semiestructurados para metadata flexible

---

## Ventajas de PostgreSQL en el Contexto Médico

### Integridad de Datos
```sql
-- Ejemplo de constraints que garantizan datos válidos
CHECK (role IN ('patient', 'doctor', 'admisionist', 'admin'))
CHECK (gender IN ('M','F','O'))
CHECK (blood_type IN ('A+','A-','B+','B-','AB+','AB-','O+','O-'))
```

### Rendimiento y Escalabilidad
- **Índices estratégicos** para consultas frecuentes
- **Particionamiento implícito** mediante fechas
- **Transacciones ACID** para operaciones críticas

### Seguridad y Auditoría
- **Soft delete** para mantener historial completo
- **Audit logs** para trazabilidad de cambios
- **Foreign keys** para integridad referencial

---

## Esquema Detallado de Tablas

### 1. `hospitals` - Hospitales y Centros Médicos

**Propósito**: Almacena la información de instituciones médicas.

```sql
CREATE TABLE hospitals (
    id_hospital SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    address VARCHAR(200) NOT NULL
);
```

**Campos**:
- `id_hospital`: Identificador único auto-incremental
- `name`: Nombre oficial del hospital (máximo 100 caracteres)
- `address`: Dirección física completa del hospital

**Relaciones**:
- Relacionado con `admisionists` para asignar personal por hospital

### 2. `users` - Sistema de Usuarios y Autenticación

**Propósito**: Gestión centralizada de usuarios, roles y autenticación.

```sql
CREATE TABLE users (
    id_user SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role VARCHAR(20) NOT NULL CHECK (role IN ('patient', 'doctor', 'admisionist', 'admin')),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);
```

**Campos**:
- `id_user`: Identificador único auto-incremental
- `username`: Nombre de usuario único para login (case-sensitive)
- `password_hash`: Hash de contraseña usando bcrypt/argon2
- `role`: Rol del sistema con validación estricta
- `created_at`: Timestamp de creación automático
- `updated_at`: Timestamp de última modificación
- `deleted_at`: Timestamp de eliminación lógica (NULL si activo)

**Constraints**:
- `username` debe ser único en todo el sistema
- `role` solo permite valores específicos del dominio

### 3. `areas` - Áreas Médicas del Hospital

**Propósito**: Catálogo de áreas o departamentos médicos.

```sql
CREATE TABLE areas (
    id_area SERIAL PRIMARY KEY,
    area_name VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);
```

**Campos**:
- `id_area`: Identificador único del área médica
- `area_name`: Nombre del área (Cardiología, Pediatría, etc.)
- Campos de auditoría estándar

### 4. `specialities` - Especialidades Médicas

**Propósito**: Catálogo de especialidades de los doctores.

```sql
CREATE TABLE specialities (
    id_speciality SERIAL PRIMARY KEY,
    speciality_name VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);
```

### 5. `services` - Servicios Médicos Ofrecidos

**Propósito**: Catálogo de servicios médicos disponibles.

```sql
CREATE TABLE services (
    id_service SERIAL PRIMARY KEY,
    service_name VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);
```

### 6. `patients` - Registro Completo de Pacientes

**Propósito**: Almacena toda la información demográfica, de contacto y médica de los pacientes.

```sql
CREATE TABLE patients (
    id_patient SERIAL PRIMARY KEY,
    id_user INT NULL REFERENCES users(id_user),
    identity_number VARCHAR(20) UNIQUE NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    second_name VARCHAR(50),
    first_lastname VARCHAR(50) NOT NULL,
    second_lastname VARCHAR(50),
    gender VARCHAR(10) CHECK (gender IN ('M','F','O')),
    birthdate DATE NOT NULL,
    blood_type VARCHAR(5) CHECK (blood_type IN ('A+','A-','B+','B-','AB+','AB-','O+','O-')),
    phone VARCHAR(20),
    email VARCHAR(100) UNIQUE,
    address VARCHAR(300),
    emergency_contact_name VARCHAR(100),
    emergency_contact_phone VARCHAR(20),
    allergies TEXT,
    current_medications TEXT,
    medical_background TEXT,
    priority INT DEFAULT 0,
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);
```

**Campos Detallados**:

**Información de Identificación**:
- `identity_number`: Cédula o documento de identidad único
- `first_name`, `second_name`, `first_lastname`, `second_lastname`: Nombre completo

**Información Demográfica**:
- `gender`: Género con validación (M, F, O)
- `birthdate`: Fecha de nacimiento para cálculo de edad
- `blood_type`: Tipo de sangre con validación estricta

**Información de Contacto**:
- `phone`: Teléfono en formato internacional
- `email`: Correo electrónico único
- `address`: Dirección residencial completa

**Información Médica Básica**:
- `allergies`: Lista de alergias conocidas
- `current_medications`: Medicamentos actuales del paciente
- `medical_background`: Antecedentes médicos relevantes

**Campos de Gestión**:
- `priority`: Nivel de prioridad (0 = normal, mayor = más urgente)
- `status`: Estado del paciente en el sistema

### 7. `doctors` - Registro de Profesionales Médicos

**Propósito**: Información completa de los doctores y su especialización.

```sql
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
```

**Relaciones Clave**:
- `id_area`: Área médica principal del doctor
- `id_speciality`: Especialidad específica
- `id_service`: Servicio que ofrece
- `id_user`: Usuario del sistema asociado

### 8. `admisionists` - Personal Administrativo

**Propósito**: Registro de personal de admisión y recepción.

```sql
CREATE TABLE admisionists (
    id_admisionist SERIAL PRIMARY KEY,
    id_user INT NOT NULL REFERENCES users(id_user) ON DELETE CASCADE,
    id_hospital INT REFERENCES hospitals(id_hospital),
    first_name VARCHAR(50) NOT NULL,
    second_name VARCHAR(50),
    first_lastname VARCHAR(50) NOT NULL,
    second_lastname VARCHAR(50),
    phone VARCHAR(20),
    email VARCHAR(100) UNIQUE,
    shift VARCHAR(50),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);
```

**Características Especiales**:
- `ON DELETE CASCADE` para eliminación automática cuando se elimina el usuario
- `shift`: Turno de trabajo (mañana, tarde, noche)

### 9. `medical_appointments` - Gestión de Citas Médicas

**Propósito**: Sistema completo de programación y seguimiento de citas.

```sql
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
    status VARCHAR(20) DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);
```

**Estados de Cita**:
- `pending`: Cita programada pero no confirmada
- `confirmed`: Cita confirmada por paciente/doctor
- `completed`: Cita atendida y finalizada
- `canceled`: Cita cancelada

**Campos de Ubicación**:
- `building`: Edificio donde se atiende la cita
- `room`: Sala o consultorio específico

### 10. `medical_history` - Historial Clínico Electrónico

**Propósito**: Registro cronológico de diagnósticos y tratamientos.

```sql
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
```

**Características**:
- Sin `deleted_at` - Los registros médicos son permanentes
- `record_date`: Fecha del evento médico (puede diferir de created_at)

### 11. `lab_results` - Resultados de Laboratorio

**Propósito**: Almacenamiento de exámenes y pruebas de laboratorio.

```sql
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
```

### 12. `notifications` - Sistema de Notificaciones

**Propósito**: Gestión de alertas y notificaciones del sistema.

```sql
CREATE TABLE notifications (
    id_notification SERIAL PRIMARY KEY,
    id_user INT NOT NULL REFERENCES users(id_user),
    title VARCHAR(200) NOT NULL,
    message TEXT NOT NULL,
    type VARCHAR(50),
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    read_at TIMESTAMP
);
```

**Tipos de Notificaciones**:
- `cita`: Recordatorios y cambios de citas
- `resultado`: Resultados de laboratorio disponibles
- `general`: Notificaciones administrativas

### 13. `medical_documents` - Gestión Documental Médica

**Propósito**: Almacenamiento de documentos, imágenes y archivos médicos.

```sql
CREATE TABLE medical_documents (
    id_document SERIAL PRIMARY KEY,
    id_patient INT NOT NULL REFERENCES patients(id_patient),
    id_doctor INT REFERENCES doctors(id_doctor),
    document_type VARCHAR(100),
    file_path TEXT NOT NULL,
    description TEXT,
    uploaded_at TIMESTAMP DEFAULT NOW()
);
```

**Tipos de Documentos**:
- `imagen`: Radiografías, resonancias, etc.
- `PDF`: Reportes, historiales
- `radiografía`: Imágenes médicas específicas

### 14. `virtual_turns` - Sistema de Turnos Virtuales

**Propósito**: Gestión de turnos para consulta externa y servicios.

```sql
CREATE TABLE virtual_turns (
  id_turn SERIAL PRIMARY KEY,
  id_patient INT NULL REFERENCES patients(id_patient) ON DELETE SET NULL,
  id_service INT NOT NULL REFERENCES services(id_service) ON DELETE CASCADE,
  id_area INT NULL REFERENCES areas(id_area) ON DELETE SET NULL,
  id_desk INT NULL REFERENCES users(id_user) ON DELETE SET NULL,
  turn_number INT NOT NULL,
  priority INT DEFAULT 0,
  status VARCHAR(20) NOT NULL DEFAULT 'waiting', 
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  called_at TIMESTAMP NULL,
  started_at TIMESTAMP NULL,
  completed_at TIMESTAMP NULL,
  expires_at TIMESTAMP NULL,
  metadata JSONB DEFAULT '{}'::jsonb,
  turn_date DATE GENERATED ALWAYS AS (created_at::date) STORED,
  CONSTRAINT unique_turn_number_per_service_per_day UNIQUE (id_service, turn_number, turn_date)
);
```

**Características Avanzadas**:

**Columna Generada**:
```sql
turn_date DATE GENERATED ALWAYS AS (created_at::date) STORED
```
- Calculada automáticamente a partir de `created_at`
- Optimizada para consultas por fecha

**Restricción de Unicidad**:
```sql
CONSTRAINT unique_turn_number_per_service_per_day UNIQUE (id_service, turn_number, turn_date)
```
- Garantiza números de turno únicos por servicio y día

**Estados del Turno**:
- `waiting`: En espera de ser llamado
- `called`: Fue llamado por recepción
- `in_progress`: Siendo atendido
- `completed`: Atención finalizada
- `expired`: Turno expirado

**Índices Optimizados**:
```sql
CREATE INDEX idx_turns_service_status ON virtual_turns (id_service, status, created_at);
CREATE INDEX idx_turns_patient ON virtual_turns (id_patient);
CREATE INDEX idx_turns_service_turn ON virtual_turns (id_service, turn_number);
```

### 15. `device_tokens` - Notificaciones Push

**Propósito**: Gestión de tokens para notificaciones móviles.

```sql
CREATE TABLE device_tokens (
  id_token SERIAL PRIMARY KEY,
  id_user INT NOT NULL REFERENCES users(id_user) ON DELETE CASCADE,
  token TEXT NOT NULL,
  platform VARCHAR(10) NOT NULL CHECK (platform IN ('fcm', 'apns')),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  last_used TIMESTAMP NULL
);

CREATE UNIQUE INDEX idx_user_token_platform ON device_tokens (id_user, token, platform);
```

**Plataformas Soportadas**:
- `fcm`: Firebase Cloud Messaging (Android)
- `apns`: Apple Push Notification Service (iOS)

### 16. `turn_audit_logs` - Auditoría de Turnos

**Propósito**: Tracking completo de cambios de estado en turnos.

```sql
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
```

### 17. `audit_logs` - Auditoría General del Sistema

**Propósito**: Registro de cambios críticos en todas las tablas del sistema.

```sql
CREATE TABLE audit_logs (
    id_audit SERIAL PRIMARY KEY,
    table_name VARCHAR(50) NOT NULL,
    record_id INT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by INT REFERENCES users(id_user),
    performed_at TIMESTAMP DEFAULT NOW()
);
```

**Ventajas del Diseño**:
- **JSONB**: Almacena datos antiguos y nuevos en formato flexible
- **Genérico**: Funciona para cualquier tabla del sistema
- **Completo**: Captura el estado completo antes y después del cambio

---

## Relaciones y Cardinalidades

### Diagrama de Relaciones Principales

```
users (1) ←→ (1) patients
users (1) ←→ (1) doctors  
users (1) ←→ (1) admisionists

doctors (N) ←→ (1) areas
doctors (N) ←→ (1) specialities
doctors (N) ←→ (1) services

patients (1) ←→ (N) medical_appointments
doctors (1) ←→ (N) medical_appointments

patients (1) ←→ (N) medical_history
patients (1) ←→ (N) lab_results
patients (1) ←→ (N) medical_documents

users (1) ←→ (N) notifications
users (1) ←→ (N) device_tokens
```

### Relaciones Detalladas

#### 1. **Sistema de Usuarios (`users`)**
- **Relación 1:1 con `patients`**: Cada paciente puede tener un usuario para acceso al sistema
- **Relación 1:1 con `doctors`**: Cada doctor tiene credenciales de acceso
- **Relación 1:1 con `admisionists`**: Personal administrativo con acceso al sistema
- **Relación 1:N con `notifications`**: Los usuarios reciben múltiples notificaciones
- **Relación 1:N con `audit_logs`**: Los usuarios realizan acciones auditadas

#### 2. **Gestión Médica (`patients` - `doctors`)**
- **Relación N:M a través de `medical_appointments`**: Pacientes y doctores se relacionan mediante citas
- **Relación 1:N de `patients` a `medical_history`**: Cada paciente tiene múltiples registros médicos
- **Relación 1:N de `doctors` a `medical_history`**: Los doctores crean múltiples registros

#### 3. **Catálogos del Sistema (`areas`, `services`, `specialities`)**
- **Relación 1:N con `doctors`**: Cada doctor pertenece a un área, servicio y especialidad
- **Relación 1:N con `medical_appointments`**: Las citas se asignan a áreas y servicios específicos

---

## Índices y Optimización

### Índices Creados

```sql
-- Para turnos virtuales
CREATE INDEX idx_turns_service_status ON virtual_turns (id_service, status, created_at);
CREATE INDEX idx_turns_patient ON virtual_turns (id_patient);
CREATE INDEX idx_turns_service_turn ON virtual_turns (id_service, turn_number);

-- Para auditoría de turnos
CREATE INDEX idx_turn_audit_turn ON turn_audit_logs (id_turn, changed_at);

-- Para device tokens
CREATE UNIQUE INDEX idx_user_token_platform ON device_tokens (id_user, token, platform);
```

### Índices Implícitos
- **Primary Keys**: Todos los campos `SERIAL PRIMARY KEY` tienen índices automáticos
- **Foreign Keys**: PostgreSQL crea índices para algunas foreign keys
- **Unique Constraints**: Campos `UNIQUE` tienen índices únicos automáticos

---

## Políticas de Eliminación y Integridad

### ON DELETE Strategies

```sql
-- Eliminación en cascada (dependencias fuertes)
ON DELETE CASCADE

-- Set null (relaciones opcionales)
ON DELETE SET NULL

-- Restrict (eliminación preventiva)
-- Comportamiento por defecto de PostgreSQL
```

### Soft Delete Pattern
Todas las entidades principales implementan soft delete mediante `deleted_at`:
- `deleted_at IS NULL` = Registro activo
- `deleted_at NOT NULL` = Registro eliminado

---

## Consideraciones de Seguridad

### Validación de Datos
- **CHECK constraints** para dominios restringidos (género, tipo de sangre, roles)
- **UNIQUE constraints** para evitar duplicados críticos
- **NOT NULL** para campos obligatorios

### Integridad Referencial
- **Foreign keys** en todas las relaciones
- **Transacciones** para operaciones complejas
- **Auditoría** para cambios críticos

---

## Flujos de Datos Típicos

### 1. Registro de Nuevo Paciente
```sql
-- 1. Crear usuario
INSERT INTO users (username, password_hash, role) VALUES (...);

-- 2. Crear paciente
INSERT INTO patients (id_user, identity_number, first_name, ...) VALUES (...);
```

### 2. Programación de Cita
```sql
-- 1. Verificar disponibilidad
SELECT * FROM medical_appointments 
WHERE id_doctor = ? AND appointment_datetime = ? AND deleted_at IS NULL;

-- 2. Crear cita
INSERT INTO medical_appointments (id_patient, id_doctor, ...) VALUES (...);
```

### 3. Sistema de Turnos
```sql
-- 1. Obtener próximo número de turno
SELECT COALESCE(MAX(turn_number), 0) + 1 
FROM virtual_turns 
WHERE id_service = ? AND turn_date = CURRENT_DATE;

-- 2. Crear turno
INSERT INTO virtual_turns (id_patient, id_service, turn_number, ...) VALUES (...);
```

### 4. Auditoría de Cambios
```sql
-- Trigger example para auditoría automática
-- (Implementado a nivel de aplicación en este caso)
INSERT INTO audit_logs (table_name, record_id, action, old_data, new_data, performed_by)
VALUES ('patients', 123, 'update', '{"name": "old"}', '{"name": "new"}', 1);
```

---

## Mantenimiento y Optimización

### Consultas de Mantenimiento Recomendadas

```sql
-- Limpiar registros eliminados antiguos (opcional)
DELETE FROM table_name WHERE deleted_at < NOW() - INTERVAL '7 years';

-- Reindexar tablas críticas
REINDEX TABLE medical_appointments;
REINDEX TABLE virtual_turns;

-- Estadísticas de uso
ANALYZE;
```

### Monitoreo de Performance
- **Índices de turnos**: Monitorear uso en horarios pico
- **Audit logs**: Rotación periódica o archivado
- **Conexiones**: Pool de conexiones para alta concurrencia

---

## Consideraciones de Escalabilidad

### Estrategias de Crecimiento
1. **Particionamiento**: Por fecha en tablas grandes (`medical_appointments`, `audit_logs`)
2. **Réplicas de Lectura**: Para reportes y analytics
3. **Archivado**: Movimiento de datos históricos a almacenamiento frío

### Capacidades Estimadas
- **Pacientes**: 100,000+ registros
- **Citas**: 1,000+ por día
- **Turnos**: 5,000+ por día
- **Auditoría**: 10,000+ eventos por día
