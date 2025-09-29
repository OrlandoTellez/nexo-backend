# Paciente App - API Documentation

## 📋 Descripción General

API REST para el sistema de gestión hospitalaria "Paciente App". Desarrollada en Rust con Axum y PostgreSQL para automatizar procesos de registro, seguimiento de pacientes y programación de citas médicas.

**URL Base**: `http://localhost:3000`

## 🔐 Autenticación

*Nota: La autenticación está pendiente de implementación. Actualmente los endpoints son públicos para desarrollo.*

## 📊 Esquema de la Base de Datos

### Relaciones Principales
```
Users ←→ Patients (1:1)
Users ←→ Doctors (1:1)  
Doctors → Areas, Services, Specialities (N:1)
Patients → Medical Appointments (1:N)
Doctors → Medical Appointments (1:N)
```

## 🏥 Endpoints de Pacientes

### Obtener todos los pacientes
```http
GET /patients
```

**Respuesta Exitosa (200 OK):**
```json
[
  {
    "id_patient": 1,
    "id_user": null,
    "first_name": "Samuel",
    "second_name": "Gabriel",
    "first_lastname": "Tellez",
    "second_lastname": "Houston",
    "address": "Rpto. satelite asososca casa no 135",
    "birthdate": "2005-01-06",
    "phone": "75061202",
    "email": "orlandotellsez36@gmail.com",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": null,
    "deleted_at": null
  }
]
```

### Obtener paciente por ID
```http
GET /patients/{id}
```

**Parámetros:**
- `id` (entero, requerido): ID del paciente

**Respuestas:**
- `200 OK`: Paciente encontrado
- `404 Not Found`: Paciente no existe
- `500 Internal Server Error`: Error del servidor

### Crear nuevo paciente
```http
POST /patients
Content-Type: application/json
```

**Cuerpo de la Solicitud:**
```json
{
  "identity_number": "001-120590-0001J",
  "first_name": "Juan",
  "second_name": "Carlos",
  "first_lastname": "Pérez",
  "second_lastname": "López",
  "gender": "M",
  "birthdate": "1990-05-12",
  "blood_type": "O+",
  "phone": "+50588887777",
  "email": "juan.perez@example.com",
  "address": "Colonia Centroamérica, Managua",
  "emergency_contact_name": "María Pérez",
  "emergency_contact_phone": "+50577776666",
  "allergies": "Penicilina",
  "current_medications": "Losartán",
  "medical_background": "Hipertensión",
  "priority": 1,
  "status": "active"
}

```

**Campos Requeridos:**
- `first_name` (string)
- `first_lastname` (string) 
- `birthdate` (date, formato: YYYY-MM-DD)

**Respuestas:**
- `201 Created`: Paciente creado exitosamente
- `400 Bad Request`: Datos inválidos
- `409 Conflict`: Email ya existe
- `500 Internal Server Error`: Error del servidor

### Actualizar paciente
```http
PUT /patients/{id}
Content-Type: application/json
```

**Cuerpo de la Solicitud:**
```json
{
  "first_name": "Samuel Actualizado",
  "phone": "88888888",
  "email": "nuevoemail@gmail.com"
}
```

**Nota:** Todos los campos son opcionales en la actualización.

### Eliminar paciente (Soft Delete)
```http
DELETE /patients/{id}
```

**Respuesta:** `200 OK` con los datos del paciente eliminado

## 👥 Endpoints de Usuarios

### Obtener todos los usuarios
```http
GET /users
```

**Respuesta:**
```json
[
  {
    "id_user": 1,
    "username": "orlandotellsez36",
    "password_hash": "$2a$10$5.9.1.0.3.2.5.4.6.7.8.9.1.2.3.4.5.6.7.8.9.1",
    "role": "patient",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": null,
    "deleted_at": null
  }
]
```

### Crear nuevo usuario
```http
POST /users
Content-Type: application/json
```

**Cuerpo de la Solicitud:**
```json
{
  "username": "orlandotellsez36",
  "password_hash": "$2a$10$5.9.1.0.3.2.5.4.6.7.8.9.1.2.3.4.5.6.7.8.9.1",
  "role": "patient"
}
```

**Roles Válidos:** `patient`, `doctor`, `admin`

**Nota:** La contraseña se hashea automáticamente con BCrypt

### Actualizar usuario
```http
PUT /users/{id}
Content-Type: application/json
```

### Eliminar usuario (Soft Delete)
```http
DELETE /users/{id}
```

## 🩺 Endpoints de Doctores

### Obtener todos los doctores
```http
GET /doctors
```

**Respuesta:**
```json
[
  {
    "id_doctor": 1,
    "id_area": 1,
    "id_speciality": 1,
    "id_service": 1,
    "id_user": null,
    "first_name": "Samuel",
    "second_name": "Gabriel",
    "first_lastname": "Tellez",
    "second_lastname": "Houston",
    "phone": "75061202",
    "email": "orlandotellsez36@gmail.com",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": null,
    "deleted_at": null
  }
]
```

### Crear nuevo doctor
```http
POST /doctors
Content-Type: application/json
```

**Cuerpo de la Solicitud:**
```json
{
  "id_area": 1,
  "id_service": 1,
  "id_speciality": 1,
  "id_user": null,
  "first_name": "Samuel",
  "second_name": "Gabriel",
  "first_lastname": "Tellez",
  "second_lastname": "Houston",
  "phone": "75061202",
  "email": "orlandotellsez36@gmail.com"
}
```

**Prerrequisitos:** Deben existir previamente el área, servicio y especialidad referenciados.

### Actualizar doctor
```http
PUT /doctors/{id}
Content-Type: application/json
```

### Eliminar doctor (Soft Delete)
```http
DELETE /doctors/{id}
```

## 🏥 Endpoints de Servicios Médicos

### Obtener todos los servicios
```http
GET /services
```

**Respuesta:**
```json
[
  {
    "id_service": 1,
    "service_name": "Servicio 1",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": null,
    "deleted_at": null
  }
]
```

### Crear nuevo servicio
```http
POST /services
Content-Type: application/json
```

**Cuerpo de la Solicitud:**
```json
{
  "service_name": "Servicio 1"
}
```

### Actualizar servicio
```http
PATCH /services/{id}
Content-Type: application/json
```

### Eliminar servicio (Soft Delete)
```http
DELETE /services/{id}
```

## 📚 Endpoints de Especialidades Médicas

### Obtener todas las especialidades
```http
GET /specialities
```

**Respuesta:**
```json
[
  {
    "id_speciality": 1,
    "speciality_name": "Especialidad 1",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": null,
    "deleted_at": null
  }
]
```

### Crear nueva especialidad
```http
POST /specialities
Content-Type: application/json
```

**Cuerpo de la Solicitud:**
```json
{
  "speciality_name": "Especialidad 1"
}
```

### Actualizar especialidad
```http
PATCH /specialities/{id}
Content-Type: application/json
```

### Eliminar especialidad (Soft Delete)
```http
DELETE /specialities/{id}
```

## ⚠️ Códigos de Estado HTTP

| Código | Descripción |
|--------|-------------|
| `200` | OK - Solicitud exitosa |
| `201` | Created - Recurso creado exitosamente |
| `400` | Bad Request - Datos de entrada inválidos |
| `404` | Not Found - Recurso no encontrado |
| `409` | Conflict - Violación de restricción única (ej: email duplicado) |
| `500` | Internal Server Error - Error del servidor |

## 🔄 Convenciones de la API

### Soft Delete
Todos los endpoints de eliminación realizan **soft delete**, marcando el registro con `deleted_at` en lugar de eliminarlo físicamente.

### Campos de Auditoría
Cada entidad incluye campos de auditoría:
- `created_at`: Fecha de creación
- `updated_at`: Fecha de última actualización
- `deleted_at`: Fecha de eliminación (null si no está eliminado)

### Formatos de Fecha
- **Fecha**: `YYYY-MM-DD` (ej: 2005-01-06)
- **Fecha/Hora**: ISO 8601 (ej: 2024-01-15T10:30:00Z)

### Validaciones
- **Email**: Formato válido y único en el sistema
- **Campos requeridos**: Validados a nivel de base de datos y aplicación
- **Relaciones foreign key**: Validadas en la base de datos

## 🚀 Ejemplo de Flujo de Uso

### 1. Crear Servicios y Especialidades Primero
```http
POST /services
{"service_name": "Medicina General"}

POST /specialities 
{"speciality_name": "Cardiología"}
```

### 2. Crear Usuario para Doctor
```http
POST /users
{
  "username": "drgarcia",
  "password_hash": "hashed_password",
  "role": "doctor"
}
```

### 3. Crear Doctor
```http
POST /doctors
{
  "id_area": 1,
  "id_service": 1,
  "id_speciality": 1,
  "id_user": 1,
  "first_name": "Carlos",
  "first_lastname": "García",
  "email": "drgarcia@hospital.gob.ni"
}
```

### 4. Crear Paciente
```http
POST /patients
{
  "first_name": "Ana",
  "first_lastname": "Martínez", 
  "birthdate": "1990-05-15",
  "email": "ana.martinez@email.com"
}
```

## 📞 Soporte y Errores

En caso de errores, la API retorna mensajes descriptivos en el cuerpo de la respuesta:

```json
{
  "error": "Descripción del error específico"
}
```

**Errores comunes:**
- `El correo ya existe`: Violación de unicidad de email
- `Error en la base de datos`: Problemas de conexión o consulta
- `Recurso no encontrado`: ID proporcionado no existe

---
