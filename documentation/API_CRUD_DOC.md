# Documentación Completa de la API

## Descripción General

API REST para el sistema de gestión hospitalaria "Paciente App". Desarrollada en Rust con Axum y PostgreSQL para automatizar procesos de registro, seguimiento de pacientes y programación de citas médicas.

**URL Base**: `http://localhost:3000` este link solo es válido para desarrollo

## Autenticación

### Sistema de Autenticación JWT

La API utiliza JSON Web Tokens (JWT) para autenticación. Los tokens se envían en una cookie HTTP-only para mayor seguridad.

### Endpoints de Autenticación

#### Iniciar Sesión
```http
POST /auth/login
Content-Type: application/json
```

**Cuerpo de la Solicitud:**
```json
{
  "username": "usuario123",
  "password": "contraseña123"
}
```

**Respuesta Exitosa (200 OK):**
```json
{
  "message": "Login exitoso",
  "success": true,
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": 1,
    "username": "usuario123",
    "role": "patient",
    "first_name": "Juan",
    "last_name": "Pérez",
    "email": "juan@example.com",
    "phone": "+50588887777"
  }
}
```

**Headers de Respuesta:**
```http
Set-Cookie: auth_token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=3600
```

**Respuestas de Error:**
- `401 Unauthorized`: Credenciales incorrectas
- `500 Internal Server Error`: Error interno del servidor

#### Cerrar Sesión
```http
POST /auth/logout
```

**Respuesta Exitosa (200 OK):**
```json
"Logout exitoso"
```

**Headers de Respuesta:**
```http
Set-Cookie: auth_token=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0
```

## Esquema de la Base de Datos

### Relaciones Principales
```
Users (1:1) Patients
Users (1:1) Doctors
Users (1:1) Admisionists
Doctors (N:1) Areas, Services, Specialities
Patients (1:N) Medical Appointments
Doctors (1:N) Medical Appointments
Patients (1:N) Medical History
Patients (1:N) Lab Results
```

## Endpoints de Gestión de Pacientes

### Obtener Todos los Pacientes
```http
GET /patients
```

**Descripción:** Recupera una lista paginada de todos los pacientes activos en el sistema.

**Parámetros de Consulta:**
- `page` (opcional): Número de página (default: 1)
- `limit` (opcional): Cantidad de registros por página (default: 20)
- `search` (opcional): Término de búsqueda en nombre, apellido o email

**Headers Requeridos:**
```http
Authorization: Bearer <jwt_token>
Content-Type: application/json
```

**Respuesta Exitosa (200 OK):**
```json
{
  "data": [
    {
      "id_patient": 1,
      "id_user": 123,
      "identity_number": "001-120590-0001J",
      "first_name": "Samuel",
      "second_name": "Gabriel",
      "first_lastname": "Tellez",
      "second_lastname": "Houston",
      "gender": "M",
      "birthdate": "2005-01-06",
      "blood_type": "O+",
      "phone": "+50575061202",
      "email": "orlandotellsez36@gmail.com",
      "address": "Rpto. satelite asososca casa no 135",
      "emergency_contact_name": "María Pérez",
      "emergency_contact_phone": "+50577776666",
      "allergies": "Penicilina",
      "current_medications": "Losartán",
      "medical_background": "Hipertensión",
      "priority": 1,
      "status": "active",
      "created_at": "2024-01-15T10:30:00Z",
      "updated_at": null,
      "deleted_at": null
    }
  ],
  "pagination": {
    "current_page": 1,
    "total_pages": 5,
    "total_records": 95,
    "per_page": 20
  }
}
```

### Obtener Paciente por ID
```http
GET /patients/{id}
```

**Descripción:** Recupera la información detallada de un paciente específico.

**Parámetros de Ruta:**
- `id` (entero, requerido): ID único del paciente

**Respuestas:**
- `200 OK`: Paciente encontrado exitosamente
- `404 Not Found`: No existe paciente con el ID proporcionado
- `500 Internal Server Error`: Error interno del servidor

**Respuesta Exitosa (200 OK):**
```json
{
  "id_patient": 1,
  "id_user": 123,
  "identity_number": "001-120590-0001J",
  "first_name": "Samuel",
  "second_name": "Gabriel",
  "first_lastname": "Tellez",
  "second_lastname": "Houston",
  "gender": "M",
  "birthdate": "2005-01-06",
  "blood_type": "O+",
  "phone": "+50575061202",
  "email": "orlandotellsez36@gmail.com",
  "address": "Rpto. satelite asososca casa no 135",
  "emergency_contact_name": "María Pérez",
  "emergency_contact_phone": "+50577776666",
  "allergies": "Penicilina",
  "current_medications": "Losartán",
  "medical_background": "Hipertensión",
  "priority": 1,
  "status": "active",
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": null,
  "deleted_at": null
}
```

### Crear Nuevo Paciente
```http
POST /patients
Content-Type: application/json
```

**Descripción:** Crea un nuevo registro de paciente en el sistema. Automáticamente genera un usuario asociado con una contraseña basada en los datos del paciente.

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
- `identity_number` (string): Cédula o documento de identidad
- `first_name` (string): Primer nombre
- `first_lastname` (string): Primer apellido
- `birthdate` (date): Fecha de nacimiento en formato YYYY-MM-DD

**Campos Opcionales:**
- `second_name` (string): Segundo nombre
- `second_lastname` (string): Segundo apellido
- `gender` (string): Género (M, F, O)
- `blood_type` (string): Tipo de sangre (A+, A-, B+, B-, AB+, AB-, O+, O-)
- `phone` (string): Teléfono en formato internacional
- `email` (string): Correo electrónico válido
- `address` (string): Dirección completa
- `emergency_contact_name` (string): Nombre del contacto de emergencia
- `emergency_contact_phone` (string): Teléfono del contacto de emergencia
- `allergies` (string): Alergias conocidas
- `current_medications` (string): Medicamentos actuales
- `medical_background` (string): Antecedentes médicos
- `priority` (integer): Nivel de prioridad (0-10)
- `status` (string): Estado del paciente (active, inactive)

**Validaciones:**
- `email`: Debe ser un email válido y único en el sistema
- `phone`: Debe seguir formato internacional E.164 (+50588887777)
- `identity_number`: Debe ser único en el sistema
- `gender`: Solo permite 'M', 'F', u 'O'
- `blood_type`: Solo permite tipos de sangre válidos

**Respuestas:**
- `201 Created`: Paciente creado exitosamente
- `400 Bad Request`: Datos de entrada inválidos o validación fallida
- `409 Conflict`: Email o número de identidad ya existen
- `500 Internal Server Error`: Error del servidor

**Respuesta Exitosa (201 Created):**
```json
{
  "id_patient": 2,
  "id_user": 124,
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
  "status": "active",
  "created_at": "2024-01-15T11:30:00Z",
  "updated_at": null,
  "deleted_at": null
}
```

### Actualizar Paciente
```http
PATCH /patients/{id}
Content-Type: application/json
```

**Descripción:** Actualiza parcialmente la información de un paciente existente.

**Parámetros de Ruta:**
- `id` (entero, requerido): ID único del paciente a actualizar

**Cuerpo de la Solicitud:**
```json
{
  "first_name": "Samuel Actualizado",
  "phone": "+50588888888",
  "email": "nuevoemail@gmail.com",
  "address": "Nueva dirección actualizada",
  "priority": 2
}
```

**Nota:** Todos los campos son opcionales. Solo se actualizarán los campos proporcionados.

**Respuestas:**
- `200 OK`: Paciente actualizado exitosamente
- `404 Not Found`: Paciente no encontrado
- `400 Bad Request`: Datos de entrada inválidos
- `409 Conflict`: Email ya existe (si se está actualizando el email)
- `500 Internal Server Error`: Error del servidor

### Eliminar Paciente (Soft Delete)
```http
DELETE /patients/{id}
```

**Descripción:** Realiza una eliminación lógica del paciente, marcándolo como eliminado pero manteniendo el registro en la base de datos.

**Parámetros de Ruta:**
- `id` (entero, requerido): ID único del paciente a eliminar

**Respuestas:**
- `200 OK`: Paciente eliminado exitosamente
- `404 Not Found`: Paciente no encontrado
- `500 Internal Server Error`: Error del servidor

**Respuesta Exitosa (200 OK):**
```json
{
  "id_patient": 1,
  "id_user": 123,
  "first_name": "Samuel",
  "first_lastname": "Tellez",
  "email": "orlandotellsez36@gmail.com",
  "deleted_at": "2024-01-15T12:30:00Z"
}
```

## Endpoints de Gestión de Usuarios

### Obtener Todos los Usuarios
```http
GET /users
```

**Descripción:** Recupera una lista de todos los usuarios del sistema (solo accesible para administradores).

**Headers Requeridos:**
```http
Authorization: Bearer <jwt_token>
Content-Type: application/json
```

**Respuesta Exitosa (200 OK):**
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
  },
  {
    "id_user": 2,
    "username": "drgarcia",
    "password_hash": "$2a$10$5.9.1.0.3.2.5.4.6.7.8.9.1.2.3.4.5.6.7.8.9.2",
    "role": "doctor",
    "created_at": "2024-01-15T11:30:00Z",
    "updated_at": null,
    "deleted_at": null
  }
]
```

### Crear Nuevo Usuario
```http
POST /users
Content-Type: application/json
```

**Descripción:** Crea un nuevo usuario en el sistema.

**Cuerpo de la Solicitud:**
```json
{
  "username": "nuevousuario",
  "password_hash": "contraseña123",
  "role": "doctor"
}
```

**Campos Requeridos:**
- `username` (string): Nombre de usuario único
- `password_hash` (string): Contraseña en texto plano (se hashea automáticamente)
- `role` (string): Rol del usuario (patient, doctor, admin, admisionist)

**Validaciones:**
- `username`: Debe tener al menos 3 caracteres y ser único
- `password_hash`: Debe tener al menos 6 caracteres
- `role`: Debe ser uno de: patient, doctor, admin, admisionist

**Respuestas:**
- `201 Created`: Usuario creado exitosamente
- `400 Bad Request`: Datos inválidos
- `409 Conflict`: Username ya existe
- `500 Internal Server Error`: Error del servidor

## Endpoints de Gestión de Doctores

### Obtener Todos los Doctores
```http
GET /doctors
```

**Descripción:** Recupera una lista de todos los doctores activos en el sistema.

**Respuesta Exitosa (200 OK):**
```json
[
  {
    "id_doctor": 1,
    "id_area": 1,
    "id_speciality": 1,
    "id_service": 1,
    "id_user": 2,
    "first_name": "Samuel",
    "second_name": "Gabriel",
    "first_lastname": "Tellez",
    "second_lastname": "Houston",
    "phone": "+50575061202",
    "email": "orlandotellsez36@gmail.com",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": null,
    "deleted_at": null
  }
]
```

### Crear Nuevo Doctor
```http
POST /doctors
Content-Type: application/json
```

**Descripción:** Crea un nuevo registro de doctor en el sistema.

**Cuerpo de la Solicitud:**
```json
{
  "id_area": 1,
  "id_service": 1,
  "id_speciality": 1,
  "id_user": 2,
  "first_name": "Carlos",
  "second_name": "Enrique",
  "first_lastname": "García",
  "second_lastname": "López",
  "phone": "+50588889999",
  "email": "drgarcia@hospital.gob.ni"
}
```

**Prerrequisitos:** Deben existir previamente el área, servicio y especialidad referenciados.

**Campos Requeridos:**
- `id_area` (integer): ID del área médica
- `id_service` (integer): ID del servicio médico
- `first_name` (string): Primer nombre
- `first_lastname` (string): Primer apellido

**Validaciones:**
- `email`: Debe ser un email válido y único
- `phone`: Formato internacional E.164

## Endpoints de Citas Médicas

### Obtener Todas las Citas
```http
GET /appointments
```

**Descripción:** Recupera una lista de todas las citas médicas.

**Parámetros de Consulta:**
- `status` (opcional): Filtrar por estado (pending, confirmed, completed, canceled)
- `date` (opcional): Filtrar por fecha específica (YYYY-MM-DD)
- `patient_id` (opcional): Filtrar por ID de paciente
- `doctor_id` (opcional): Filtrar por ID de doctor

### Crear Nueva Cita Médica
```http
POST /appointments
Content-Type: application/json
```

**Descripción:** Programa una nueva cita médica.

**Cuerpo de la Solicitud:**
```json
{
  "id_patient": 1,
  "id_doctor": 1,
  "id_area": 1,
  "id_service": 1,
  "appointment_datetime": "2024-02-01T10:00:00Z",
  "building": "A",
  "room": "101",
  "notes": "Paciente con síntomas de gripe"
}
```

**Campos Requeridos:**
- `id_patient` (integer): ID del paciente
- `id_doctor` (integer): ID del doctor
- `id_area` (integer): ID del área
- `id_service` (integer): ID del servicio
- `appointment_datetime` (string): Fecha y hora de la cita en ISO 8601

## Endpoints de Historial Médico

### Obtener Historial Médico
```http
GET /medical_history
```

**Descripción:** Recupera el historial médico de pacientes.

**Parámetros de Consulta:**
- `patient_id` (opcional): Filtrar por ID de paciente

### Crear Registro de Historial Médico
```http
POST /medical_history
Content-Type: application/json
```

**Cuerpo de la Solicitud:**
```json
{
  "id_patient": 1,
  "id_doctor": 1,
  "diagnosis": "Hipertensión arterial estado I",
  "treatment": "Losartán 50mg cada 24 horas",
  "notes": "Controlar presión arterial semanalmente"
}
```

## Endpoints de Resultados de Laboratorio

### Obtener Resultados de Laboratorio
```http
GET /lab_results
```

**Descripción:** Recupera resultados de laboratorio.

### Crear Resultado de Laboratorio
```http
POST /lab_results
Content-Type: application/json
```

**Cuerpo de la Solicitud:**
```json
{
  "id_patient": 1,
  "id_doctor": 1,
  "lab_name": "Laboratorio Central",
  "test_type": "Hemograma completo",
  "result": "Hemoglobina: 14.2 g/dL, Hematocrito: 42%"
}
```

## Códigos de Estado HTTP

| Código | Descripción | Casos de Uso |
|--------|-------------|--------------|
| `200` | OK | Solicitud exitosa, datos devueltos |
| `201` | Created | Recurso creado exitosamente |
| `400` | Bad Request | Datos de entrada inválidos o validación fallida |
| `401` | Unauthorized | Autenticación requerida o fallida |
| `403` | Forbidden | Permisos insuficientes |
| `404` | Not Found | Recurso no encontrado |
| `409` | Conflict | Violación de restricción única (email, username, etc.) |
| `500` | Internal Server Error | Error interno del servidor |

## Convenciones de la API

### Soft Delete
Todos los endpoints de eliminación realizan **soft delete**, marcando el registro con `deleted_at` en lugar de eliminarlo físicamente. Esto permite:
- Mantener la integridad referencial
- Recuperar datos si es necesario
- Auditoría completa de datos

### Campos de Auditoría
Cada entidad incluye campos de auditoría automáticos:
- `created_at`: Fecha de creación (automático)
- `updated_at`: Fecha de última actualización (automático)
- `deleted_at`: Fecha de eliminación (null si está activo)

### Paginación
Endpoints que devuelven listas implementan paginación:
```json
{
  "data": [...],
  "pagination": {
    "current_page": 1,
    "total_pages": 5,
    "total_records": 95,
    "per_page": 20
  }
}
```

### Formatos de Fecha
- **Fecha**: `YYYY-MM-DD` (ej: 2005-01-06)
- **Fecha/Hora**: ISO 8601 (ej: 2024-01-15T10:30:00Z)

### Validaciones Comunes
- **Email**: Formato válido y único en el sistema
- **Teléfono**: Formato internacional E.164 (+50588887777)
- **Campos requeridos**: Validados a nivel de aplicación y base de datos
- **Relaciones foreign key**: Validadas en la base de datos

## Ejemplo de Flujo Completo

### 1. Crear Servicios y Especialidades
```http
POST /services
{
  "service_name": "Medicina General"
}

POST /specialities 
{
  "speciality_name": "Cardiología"
}
```

### 2. Crear Usuario para Doctor
```http
POST /users
{
  "username": "drgarcia",
  "password_hash": "password123",
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
  "email": "drgarcia@hospital.gob.ni",
  "phone": "+50588889999"
}
```

### 4. Crear Paciente
```http
POST /patients
{
  "identity_number": "001-250589-1000A",
  "first_name": "Ana",
  "first_lastname": "Martínez", 
  "birthdate": "1990-05-15",
  "email": "ana.martinez@email.com",
  "phone": "+50588880000"
}
```

### 5. Programar Cita
```http
POST /appointments
{
  "id_patient": 1,
  "id_doctor": 1,
  "id_area": 1,
  "id_service": 1,
  "appointment_datetime": "2024-02-01T14:30:00Z"
}
```

## Manejo de Errores

La API retorna mensajes de error descriptivos en formato JSON:

```json
{
  "error": "Descripción específica del error",
  "code": "ERROR_CODE",
  "details": {
    "field": "email",
    "message": "El correo ya está registrado"
  }
}
```

**Errores comunes:**
- `VALIDATION_ERROR`: Errores de validación de datos
- `NOT_FOUND`: Recurso no encontrado
- `DUPLICATE_ENTRY`: Violación de unicidad
- `DATABASE_ERROR`: Error de base de datos
- `AUTH_ERROR`: Error de autenticación


