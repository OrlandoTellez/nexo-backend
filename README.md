# Paciente App Backend

Backend para el sistema de gestión de pacientes y citas médicas desarrollado en Rust como parte del hackaton 2025 Nicaragua.

## 📋 Descripción

Sistema backend que automatiza los procesos de registro, seguimiento de pacientes y programación de citas médicas para hospitales públicos de Nicaragua. Desarrollado como solución para el reto avanzado del hackaton.

## 🚀 Funcionalidades Principales

### Para Pacientes
- **Registro y gestión de perfiles** de pacientes
- **Programación y gestión de citas médicas** (solicitud, confirmación, recordatorios, consulta, cancelación, reprogramación)
- **Acceso a historial clínico básico** y notificaciones de resultados

### Para Personal Médico
- **Gestión de citas médicas** y agendas de médicos
- **Registro y gestión de expedientes clínicos** electrónicos
- **Gestión de flujo de pacientes** y administración de reportes

## 🛠 Tecnologías Utilizadas

- **Rust** - Lenguaje de programación
- **Axum** - Framework web asíncrono
- **SQLx** - ORM y cliente de base de datos
- **PostgreSQL** - Base de datos
- **Tokio** - Runtime asíncrono
- **Serde** - Serialización/Deserialización
- **BCrypt** - Hash de contraseñas

## 📁 Estructura del Proyecto

```
src/
├── main.rs               # Punto de entrada de la aplicación
├── config.rs             # Configuración (db, variables de entorno)
├── domain/               # Entidades del dominio
│   ├── hospital.rs
│   ├── patient.rs
│   ├── user.rs
│   ├── doctor.rs
│   ├── services.rs
│   └── speciality.rs
├── application/          # Lógica de negocio (servicios)
│   ├── hospital_service.rs
│   ├── patient_service.rs
│   ├── user_service.rs
│   ├── doctor_service.rs
│   ├── services_service.rs
│   └── speciality_service.rs
├── infrastructure/       # Acceso a datos (repositorios)
│   ├── hospital_repository.rs
│   ├── patient_repository.rs
│   ├── user_repository.rs
│   ├── doctor_repository.rs
│   ├── services_repository.rs
│   └── speciality_repository.rs
├── interfaces/           # Controladores HTTP
│   ├── hospital_controller.rs
│   ├── patient_controller.rs
│   ├── user_controller.rs
│   ├── doctor_controller.rs
│   ├── services_controller.rs
│   └── speciality_controller.rs
└── routes/               # Definición de rutas
    ├── hospital.rs
    ├── patient.rs
    ├── user.rs
    ├── doctor.rs
    ├── services.rs
    └── speciality.rs
```

## 🗄️ Base de Datos

## diagrama 
![diagrama entidad relacion](public/diagrama_entidad_relacion.png)

### Esquema Principal
- **hospitals** - Información de hospitales
- **users** - Usuarios del sistema (pacientes, doctores, admin)
- **patients** - Datos de pacientes
- **doctors** - Datos de doctores
- **areas** - Áreas médicas
- **services** - Servicios médicos
- **specialities** - Especialidades médicas
- **medical_appointments** - Citas médicas
- **medical_history** - Historial clínico
- **medical_documents** - Documentos médicos
- **notifications** - Notificaciones
- **lab_results** - Resultados de laboratorio
- **audit_logs** - Auditoría de cambios

### Script de Base de Datos
Ver archivo `query.sql` para el esquema completo.

## 🔧 Instalación y Configuración

### Prerrequisitos
- Rust 1.70+
- PostgreSQL 12+
- Cargo (gestor de paquetes de Rust)

### Pasos de Instalación

1. **Clonar el repositorio**
```bash
git clone https://github.com/OrlandoTellez/paciente-app-backend.git
cd paciente-app-backend
```

2. **Configurar base de datos**
```bash
# Crear la base de datos ejecutando el script query.sql
psql -U postgres -f query.sql
```

3. **Configurar variables de entorno**
archivo .env
```bash
# Editar la cadena de conexión en src/main.rs
# postgresql://usuario:contraseña@localhost:5432/db_paciente_app
```

4. **Instalar dependencias y ejecutar**
```bash
cargo build
cargo run
```

El servidor estará disponible en `http://localhost:3000`

## 📡 Endpoints de la API

### 👥 Pacientes

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/patients` | Obtener todos los pacientes |
| GET | `/patients/{id}` | Obtener paciente por ID |
| POST | `/patients` | Crear nuevo paciente |
| PATCH | `/patients/{id}` | Actualizar paciente |
| DELETE | `/patients/{id}` | Eliminar paciente (soft delete) |

**Ejemplo POST /patients:**
```json
{
  "id_user": null,
  "first_name": "Samuel",
  "second_name": "Gabriel",
  "first_lastname": "Tellez",
  "second_lastname": "Houston",
  "address": "Rpto. satelite asososca casa no 135",
  "birthdate": "2005-01-06",
  "phone": "75061202",
  "email": "orlandotellsez36@gmail.com"
}
```

### 👤 Usuarios

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/users` | Obtener todos los usuarios |
| GET | `/users/{id}` | Obtener usuario por ID |
| POST | `/users` | Crear nuevo usuario |
| PATCH | `/users/{id}` | Actualizar usuario |
| DELETE | `/users/{id}` | Eliminar usuario (soft delete) |

**Ejemplo POST /users:**
```json
{
  "username": "orlandotellsez36",
  "password_hash": "$2a$10$5.9.1.0.3.2.5.4.6.7.8.9.1.2.3.4.5.6.7.8.9.1",
  "role": "patient"
}
```

### 🩺 Doctores

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/doctors` | Obtener todos los doctores |
| GET | `/doctors/{id}` | Obtener doctor por ID |
| POST | `/doctors` | Crear nuevo doctor |
| PUT | `/doctors/{id}` | Actualizar doctor |
| DELETE | `/doctors/{id}` | Eliminar doctor (soft delete) |

**Ejemplo POST /doctors:**
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

### 🏥 Servicios Médicos

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/services` | Obtener todos los servicios |
| GET | `/services/{id}` | Obtener servicio por ID |
| POST | `/services` | Crear nuevo servicio |
| PATCH | `/services/{id}` | Actualizar servicio |
| DELETE | `/services/{id}` | Eliminar servicio (soft delete) |

### 📚 Especialidades Médicas

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/specialities` | Obtener todas las especialidades |
| GET | `/specialities/{id}` | Obtener especialidad por ID |
| POST | `/specialities` | Crear nueva especialidad |
| PATCH | `/specialities/{id}` | Actualizar especialidad |
| DELETE | `/specialities/{id}` | Eliminar especialidad (soft delete) |

## 🔐 Seguridad

- **Hash de contraseñas**: Uso de BCrypt para el almacenamiento seguro de contraseñas
- **Soft delete**: Eliminación lógica para mantener integridad de datos
- **Validación de datos**: Validación a nivel de base de datos y aplicación
- **Auditoría**: Logs de auditoría para cambios críticos

## 🚦 Flujo de Citas Médicas

1. **Registro** → Paciente se registra en el sistema
2. **Solicitud** → Paciente solicita cita médica
3. **Asignación** → Sistema asigna doctor y horario
4. **Confirmación** → Paciente y doctor confirman cita
5. **Recordatorio** → Sistema envía recordatorio
6. **Ejecución** → Cita se realiza (pendiente de implementación)
7. **Seguimiento** → Registro de resultados y seguimiento

## 🧪 Pruebas

Para ejecutar las pruebas del sistema:

```bash
cargo test
```

## 📈 Próximas Funcionalidades

- [ ] Sistema completo de citas médicas
- [ ] Historial clínico electrónico
- [ ] Sistema de notificaciones y recordatorios
- [ ] Fila virtual/turnos en tiempo real
- [ ] Reportes y dashboard administrativo
- [ ] Integración con geolocalización para unidades de salud

## 👥 Roles del Sistema

- **Paciente**: Gestión de perfil y citas
- **Doctor**: Gestión de agenda y expedientes
- **Admin**: Administración completa del sistema

## 🤝 Contribución

Este proyecto fue desarrollado como parte de un hackaton. Para contribuir:

1. Fork del proyecto
2. Crear rama de feature (`git checkout -b feature/AmazingFeature`)
3. Commit de cambios (`git commit -m 'Add AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abrir Pull Request

## 📄 Licencia

Este proyecto es desarrollado para el hackaton 2025 de Nicaragua.

---
