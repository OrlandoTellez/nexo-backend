# Documentación de la Base de Datos `db_paciente_app`

## Descripción General

La base de datos `db_paciente_app` está diseñada para soportar un **sistema de gestión de pacientes y servicios médicos**, permitiendo almacenar información de hospitales, usuarios, doctores, pacientes, citas médicas, historial clínico, resultados de laboratorio, notificaciones, documentos médicos y auditoría de cambios.

El sistema está optimizado para **PostgreSQL**, aprovechando sus ventajas de tipos de datos avanzados (`JSONB`), referencias entre tablas (`FOREIGN KEY`) y funciones de auditoría.

---

## Ventajas de usar PostgreSQL

* **Integridad referencial:** Las relaciones entre tablas aseguran consistencia de los datos.
* **Tipos de datos avanzados:** Permite `JSONB` para almacenar datos semiestructurados como auditoría.
* **Escalabilidad y rendimiento:** Maneja grandes volúmenes de datos clínicos.
* **Funciones de auditoría y seguridad:** Facilita mantener registros de cambios y permisos.
* **Soporte de transacciones:** Garantiza operaciones atómicas, importantes en gestión de citas y resultados.

---

## Tablas y Estructura

### 1. `hospitals` – Hospitales

Almacena información de hospitales donde se realizan los servicios médicos.

| Campo        | Tipo         | Restricciones | Descripción                      |
| ------------ | ------------ | ------------- | -------------------------------- |
| id\_hospital | SERIAL       | PK            | Identificador único del hospital |
| name         | VARCHAR(100) | NOT NULL      | Nombre del hospital              |
| address      | VARCHAR(200) | NOT NULL      | Dirección del hospital           |

---

### 2. `users` – Usuarios

Almacena usuarios del sistema, incluyendo pacientes, doctores y administradores.

| Campo          | Tipo        | Restricciones    | Descripción                        |
| -------------- | ----------- | ---------------- | ---------------------------------- |
| id\_user       | SERIAL      | PK               | Identificador único del usuario    |
| username       | VARCHAR(50) | UNIQUE, NOT NULL | Nombre de usuario para login       |
| password\_hash | TEXT        | NOT NULL         | Contraseña en hash                 |
| role           | VARCHAR(20) | NOT NULL         | Rol: 'patient', 'doctor', 'admin'  |
| created\_at    | TIMESTAMP   | DEFAULT NOW()    | Fecha de creación                  |
| updated\_at    | TIMESTAMP   |                  | Fecha de última actualización      |
| deleted\_at    | TIMESTAMP   |                  | Fecha de eliminación (soft delete) |

---

### 3. `areas` – Áreas Médicas

Define áreas dentro del hospital, como Cardiología, Pediatría, etc.

| Campo       | Tipo         | Restricciones    | Descripción                  |
| ----------- | ------------ | ---------------- | ---------------------------- |
| id\_area    | SERIAL       | PK               | Identificador único del área |
| area\_name  | VARCHAR(100) | UNIQUE, NOT NULL | Nombre del área              |
| created\_at | TIMESTAMP    | DEFAULT NOW()    | Fecha de creación            |
| updated\_at | TIMESTAMP    |                  | Fecha de actualización       |
| deleted\_at | TIMESTAMP    |                  | Fecha de eliminación         |

---

### 4. `specialities` – Especialidades Médicas

Define especialidades dentro de un área, como Cardiología Pediátrica, Cirugía General, etc.

| Campo            | Tipo         | Restricciones    | Descripción                            |
| ---------------- | ------------ | ---------------- | -------------------------------------- |
| id\_speciality   | SERIAL       | PK               | Identificador único de la especialidad |
| speciality\_name | VARCHAR(100) | UNIQUE, NOT NULL | Nombre de la especialidad              |
| created\_at      | TIMESTAMP    | DEFAULT NOW()    | Fecha de creación                      |
| updated\_at      | TIMESTAMP    |                  | Fecha de actualización                 |
| deleted\_at      | TIMESTAMP    |                  | Fecha de eliminación                   |

---

### 5. `services` – Servicios Médicos

Almacena los servicios que ofrece el hospital.

| Campo         | Tipo         | Restricciones    | Descripción                      |
| ------------- | ------------ | ---------------- | -------------------------------- |
| id\_service   | SERIAL       | PK               | Identificador único del servicio |
| service\_name | VARCHAR(100) | UNIQUE, NOT NULL | Nombre del servicio              |
| created\_at   | TIMESTAMP    | DEFAULT NOW()    | Fecha de creación                |
| updated\_at   | TIMESTAMP    |                  | Fecha de actualización           |
| deleted\_at   | TIMESTAMP    |                  | Fecha de eliminación             |

---

### 6. `patients` – Pacientes

Registra la información de los pacientes.

| Campo            | Tipo         | Restricciones | Descripción                        |
| ---------------- | ------------ | ------------- | ---------------------------------- |
| id\_patient      | SERIAL       | PK            | Identificador único del paciente   |
| id\_user         | INT          | FK → users    | Relación con la tabla de usuarios  |
| first\_name      | VARCHAR(50)  | NOT NULL      | Nombre del paciente                |
| second\_name     | VARCHAR(50)  |               | Segundo nombre                     |
| first\_lastname  | VARCHAR(50)  | NOT NULL      | Apellido paterno                   |
| second\_lastname | VARCHAR(50)  |               | Apellido materno                   |
| address          | VARCHAR(300) |               | Dirección del paciente             |
| birthdate        | DATE         | NOT NULL      | Fecha de nacimiento                |
| phone            | VARCHAR(20)  |               | Teléfono                           |
| email            | VARCHAR(100) | UNIQUE        | Correo electrónico                 |
| created\_at      | TIMESTAMP    | DEFAULT NOW() | Fecha de creación                  |
| updated\_at      | TIMESTAMP    |               | Fecha de actualización             |
| deleted\_at      | TIMESTAMP    |               | Fecha de eliminación (soft delete) |

---

### 7. `doctors` – Doctores

Registra información de los doctores.

| Campo            | Tipo         | Restricciones     | Descripción                    |
| ---------------- | ------------ | ----------------- | ------------------------------ |
| id\_doctor       | SERIAL       | PK                | Identificador único del doctor |
| id\_area         | INT          | FK → areas        | Área asignada                  |
| id\_speciality   | INT          | FK → specialities | Especialidad médica            |
| id\_service      | INT          | FK → services     | Servicio asignado              |
| id\_user         | INT          | FK → users        | Usuario relacionado            |
| first\_name      | VARCHAR(50)  | NOT NULL          | Nombre del doctor              |
| second\_name     | VARCHAR(50)  |                   | Segundo nombre                 |
| first\_lastname  | VARCHAR(50)  | NOT NULL          | Apellido paterno               |
| second\_lastname | VARCHAR(50)  |                   | Apellido materno               |
| phone            | VARCHAR(20)  |                   | Teléfono                       |
| email            | VARCHAR(100) | UNIQUE            | Correo electrónico             |
| created\_at      | TIMESTAMP    | DEFAULT NOW()     | Fecha de creación              |
| updated\_at      | TIMESTAMP    |                   | Fecha de actualización         |
| deleted\_at      | TIMESTAMP    |                   | Fecha de eliminación           |

---

### 8. `medical_appointments` – Citas Médicas

Almacena todas las citas médicas.

| Campo                 | Tipo        | Restricciones     | Descripción                    |
| --------------------- | ----------- | ----------------- | ------------------------------ |
| id\_appointment       | SERIAL      | PK                | Identificador único de la cita |
| id\_patient           | INT         | FK → patients     | Paciente que asiste            |
| id\_doctor            | INT         | FK → doctors      | Doctor que atiende             |
| id\_area              | INT         | FK → areas        | Área asignada                  |
| id\_service           | INT         | FK → services     | Servicio asignado              |
| appointment\_datetime | TIMESTAMP   | NOT NULL          | Fecha y hora de la cita        |
| building              | VARCHAR(10) |                   | Edificio                       |
| room                  | VARCHAR(10) |                   | Sala                           |
| notes                 | TEXT        |                   | Notas del doctor               |
| prescription          | TEXT        |                   | Prescripción del doctor        |
| status                | VARCHAR(20) | DEFAULT 'pending' | Estado de la cita              |
| created\_at           | TIMESTAMP   | DEFAULT NOW()     | Fecha de creación              |
| updated\_at           | TIMESTAMP   |                   | Fecha de actualización         |
| deleted\_at           | TIMESTAMP   |                   | Fecha de eliminación           |

---

### 9. `medical_history` – Historial Clínico

Registro de diagnóstico y tratamientos de los pacientes.

| Campo        | Tipo      | Restricciones | Descripción                |
| ------------ | --------- | ------------- | -------------------------- |
| id\_history  | SERIAL    | PK            | Identificador del registro |
| id\_patient  | INT       | FK → patients | Paciente al que pertenece  |
| id\_doctor   | INT       | FK → doctors  | Doctor que registró        |
| diagnosis    | TEXT      | NOT NULL      | Diagnóstico                |
| treatment    | TEXT      |               | Tratamiento                |
| notes        | TEXT      |               | Notas adicionales          |
| record\_date | TIMESTAMP | DEFAULT NOW() | Fecha del registro         |
| created\_at  | TIMESTAMP | DEFAULT NOW() | Fecha de creación          |
| updated\_at  | TIMESTAMP |               | Fecha de actualización     |

---

### 10. `lab_results` – Resultados de Laboratorio

Almacena resultados de pruebas de laboratorio.

| Campo        | Tipo         | Restricciones | Descripción                 |
| ------------ | ------------ | ------------- | --------------------------- |
| id\_result   | SERIAL       | PK            | Identificador del resultado |
| id\_patient  | INT          | FK → patients | Paciente al que pertenece   |
| id\_doctor   | INT          | FK → doctors  | Doctor que solicitó         |
| lab\_name    | VARCHAR(100) | NOT NULL      | Nombre del laboratorio      |
| test\_type   | VARCHAR(100) |               | Tipo de prueba              |
| result       | TEXT         | NOT NULL      | Resultado de la prueba      |
| result\_date | TIMESTAMP    | DEFAULT NOW() | Fecha del resultado         |
| created\_at  | TIMESTAMP    | DEFAULT NOW() | Fecha de                    |


creación               |
\| updated\_at   | TIMESTAMP  |               | Fecha de actualización          |

---

### 11. `notifications` – Notificaciones

Permite enviar alertas a los usuarios.

| Campo            | Tipo         | Restricciones | Descripción                      |
| ---------------- | ------------ | ------------- | -------------------------------- |
| id\_notification | SERIAL       | PK            | Identificador único              |
| id\_user         | INT          | FK → users    | Usuario que recibe               |
| title            | VARCHAR(200) | NOT NULL      | Título de la notificación        |
| message          | TEXT         | NOT NULL      | Mensaje                          |
| type             | VARCHAR(50)  |               | Tipo ('cita', 'resultado', etc.) |
| is\_read         | BOOLEAN      | DEFAULT FALSE | Si fue leído                     |
| created\_at      | TIMESTAMP    | DEFAULT NOW() | Fecha de creación                |
| read\_at         | TIMESTAMP    |               | Fecha de lectura                 |

---

### 12. `medical_documents` – Documentos Médicos

Almacena documentos, imágenes o PDF asociados a pacientes.

| Campo          | Tipo         | Restricciones | Descripción                   |
| -------------- | ------------ | ------------- | ----------------------------- |
| id\_document   | SERIAL       | PK            | Identificador único           |
| id\_patient    | INT          | FK → patients | Paciente asociado             |
| id\_doctor     | INT          | FK → doctors  | Doctor que subió el documento |
| document\_type | VARCHAR(100) |               | Tipo de documento             |
| file\_path     | TEXT         | NOT NULL      | Ruta del archivo o URL        |
| description    | TEXT         |               | Descripción del documento     |
| uploaded\_at   | TIMESTAMP    | DEFAULT NOW() | Fecha de subida               |

---

### 13. `audit_logs` – Auditoría

Registro de cambios en tablas críticas para control de datos.

| Campo         | Tipo        | Restricciones | Descripción                            |
| ------------- | ----------- | ------------- | -------------------------------------- |
| id\_audit     | SERIAL      | PK            | Identificador único                    |
| table\_name   | VARCHAR(50) | NOT NULL      | Nombre de la tabla modificada          |
| record\_id    | INT         | NOT NULL      | ID del registro modificado             |
| action        | VARCHAR(20) | NOT NULL      | Acción realizada: insert/update/delete |
| old\_data     | JSONB       |               | Datos antes del cambio                 |
| new\_data     | JSONB       |               | Datos después del cambio               |
| performed\_by | INT         | FK → users    | Usuario que realizó la acción          |
| performed\_at | TIMESTAMP   | DEFAULT NOW() | Fecha y hora de la acción              |

---
### Relaciones Clave

### 1. **Usuarios (`USERS`)**

* **Relaciones:**

  * `USERS ||--o{ PATIENTS : tiene` → Cada usuario puede ser un paciente, pero no todos los pacientes necesitan un usuario (opcional en tu diseño).
  * `USERS ||--o{ DOCTORS : tiene` → Cada usuario puede ser un doctor.
  * `USERS ||--o{ AUDIT_LOGS : realiza` → Los usuarios realizan acciones que se registran en auditoría.
  * `USERS ||--o{ NOTIFICATIONS : recibe` → Los usuarios reciben notificaciones.

**Interpretación:**
Los usuarios son la base de la autenticación y autorización; los pacientes y doctores se vinculan opcionalmente a ellos para iniciar sesión. También son los actores responsables de los cambios (auditoría) y los receptores de notificaciones.

---

### 2. **Áreas (`AREAS`)**

* `AREAS ||--o{ DOCTORS : pertenece_a` → Cada doctor pertenece a un área específica.
* `AREAS ||--o{ MEDICAL_APPOINTMENTS : asignada_a` → Cada cita médica se asigna a un área (sala o departamento).

**Interpretación:**
Las áreas permiten clasificar médicos y citas según departamentos o especialidades clínicas generales.

---

### 3. **Especialidades (`SPECIALITIES`)**

* `SPECIALITIES ||--o{ DOCTORS : pertenece_a` → Cada doctor puede tener una especialidad dentro de su área.

**Interpretación:**
Separa especialidades específicas dentro de un área médica. Por ejemplo, Cardiología dentro de Medicina Interna.

---

### 4. **Servicios (`SERVICES`)**

* `SERVICES ||--o{ DOCTORS : proporciona` → Cada doctor ofrece servicios específicos.
* `SERVICES ||--o{ MEDICAL_APPOINTMENTS : asignada_a` → Cada cita médica se asocia a un servicio.

**Interpretación:**
Los servicios permiten mapear qué tipo de atención médica se brinda y vincularlo tanto a doctores como a citas.

---

### 5. **Pacientes (`PATIENTS`)**

* `PATIENTS ||--o{ MEDICAL_APPOINTMENTS : tiene` → Cada paciente puede tener múltiples citas.
* `PATIENTS ||--o{ MEDICAL_HISTORY : tiene` → Cada paciente tiene su historial clínico.
* `PATIENTS ||--o{ LAB_RESULTS : tiene` → Cada paciente puede tener múltiples resultados de laboratorio.
* `PATIENTS ||--o{ MEDICAL_DOCUMENTS : tiene` → Cada paciente puede tener varios documentos médicos asociados.

**Interpretación:**
Los pacientes son el centro de toda la información médica; las citas, resultados, historial y documentos están vinculados directamente a ellos.

---

### 6. **Doctores (`DOCTORS`)**

* `DOCTORS ||--o{ MEDICAL_APPOINTMENTS : atiende` → Cada doctor atiende múltiples citas.
* `DOCTORS ||--o{ MEDICAL_HISTORY : registra` → Cada doctor puede registrar entradas de historial clínico.
* `DOCTORS ||--o{ LAB_RESULTS : registra` → Cada doctor puede registrar resultados de laboratorio.
* `DOCTORS ||--o{ MEDICAL_DOCUMENTS : sube` → Cada doctor puede subir documentos médicos.

**Interpretación:**
Los doctores son los responsables de la creación y gestión de registros médicos de los pacientes.

---

### 7. **Citas Médicas (`MEDICAL_APPOINTMENTS`)**

* Se vincula con:

  * Paciente (`id_patient`)
  * Doctor (`id_doctor`)
  * Área (`id_area`)
  * Servicio (`id_service`)

**Interpretación:**
Cada cita es un evento que conecta a un paciente con un doctor, dentro de un área y asociado a un servicio específico.

---

### 8. **Historial Médico (`MEDICAL_HISTORY`)**

* Se vincula con:

  * Paciente (`id_patient`)
  * Doctor (`id_doctor`)

**Interpretación:**
Permite almacenar diagnósticos, tratamientos y notas, registradas por un doctor para un paciente específico.

---

### 9. **Resultados de Laboratorio (`LAB_RESULTS`)**

* Se vincula con:

  * Paciente (`id_patient`)
  * Doctor (`id_doctor`)

**Interpretación:**
Cada resultado de laboratorio está asociado a un paciente y al doctor que lo solicitó o registró.

---

### 10. **Notificaciones (`NOTIFICATIONS`)**

* Se vincula con:

  * Usuario (`id_user`)

**Interpretación:**
Permite enviar alertas personalizadas a los usuarios del sistema, ya sean pacientes o doctores.

---

### 11. **Documentos Médicos (`MEDICAL_DOCUMENTS`)**

* Se vincula con:

  * Paciente (`id_patient`)
  * Doctor (`id_doctor`)

**Interpretación:**
Almacena documentos como imágenes, PDFs o radiografías, vinculados al paciente y subidos por un doctor.

---

### 12. **Auditoría (`AUDIT_LOGS`)**

* Se vincula con:

  * Usuario (`performed_by`)

**Interpretación:**
Registra cambios importantes en las tablas críticas, permitiendo trazabilidad de acciones de usuarios.

---
