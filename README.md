# Paciente App Backend

Backend para el sistema de gestión de pacientes y citas médicas desarrollado en Rust como parte del hackaton "Portal Paciente - Paciente App".

## 📋 Descripción

Sistema backend que automatiza los procesos de registro, seguimiento de pacientes y programación de citas médicas para hospitales públicos de Nicaragua. Desarrollado como solución para el reto avanzado del hackaton.

## 🚀 Características Principales

### ✅ Funcionalidades Implementadas

- **Gestión completa de pacientes** (CRUD con validación de datos)
- **Sistema de usuarios y autenticación** (roles: patient, doctor, admin)
- **Gestión de doctores y especialidades médicas**
- **Sistema de citas médicas** con múltiples estados
- **Historial clínico electrónico**
- **Resultados de laboratorio**
- **Validación robusta de datos** en todos los endpoints
- **Arquitectura hexagonal** (Domain-Driven Design)

### 🏥 Entidades del Sistema

- **Pacientes** - Registro y gestión de información de pacientes
- **Doctores** - Especialistas médicos con áreas y servicios
- **Usuarios** - Sistema de autenticación y roles
- **Citas Médicas** - Programación y seguimiento de consultas
- **Historial Médico** - Registro de diagnósticos y tratamientos
- **Resultados de Laboratorio** - Exámenes y pruebas médicas
- **Servicios y Especialidades** - Catálogos del sistema hospitalario

## 🛠 Tecnologías Utilizadas

### Backend Principal
- **Rust** - Lenguaje de programación (edición 2021)
- **Axum** - Framework web asíncrono
- **SQLx** - ORM y cliente de base de datos con PostgreSQL
- **Tokio** - Runtime asíncrono

### Base de Datos
- **PostgreSQL** - Base de datos principal
- **Características avanzadas**: JSONB, foreign keys, transacciones

### Seguridad y Validación
- **BCrypt** - Hash de contraseñas
- **Validator** - Validación de datos con reglas personalizadas
- **Regex** - Validación de formatos específicos (teléfonos, emails)

### Utilidades
- **Serde** - Serialización/Deserialización
- **Chrono** - Manejo de fechas y horas
- **Anyhow** - Manejo de errores
- **Async-trait** - Traits asíncronos

## 📁 Estructura del Proyecto

```
src/
├── main.rs              # Punto de entrada de la aplicación
├── config.rs            # Configuración (variables de entorno)
├── domain/              # Entidades del dominio (DDD)
│   ├── patient.rs       # Entidad Paciente
│   ├── doctor.rs        # Entidad Doctor  
│   ├── user.rs          # Entidad Usuario
│   ├── appointment.rs   # Entidad Cita Médica
│   ├── medical_history.rs # Historial médico
│   └── lab_result.rs    # Resultados de laboratorio
├── application/         # Casos de uso y lógica de negocio
│   ├── patient_service.rs
│   ├── doctor_service.rs
│   ├── user_service.rs
│   └── ... (servicios para cada entidad)
├── infrastructure/      # Adaptadores y acceso a datos
│   ├── patient_repository.rs
│   ├── doctor_repository.rs
│   └── ... (repositorios para cada entidad)
├── interfaces/          # Controladores HTTP (entrypoints)
│   ├── patient_controller.rs
│   ├── doctor_controller.rs
│   └── ... (controladores para cada entidad)
├── routes/              # Definición de rutas de la API
│   ├── patient.rs
│   ├── doctor.rs
│   └── ... (rutas para cada entidad)
└── helpers/             # Utilidades y helpers
    └── validators.rs    # Validadores personalizados
```

## 🗄️ Base de Datos

## diagrama 
![diagrama entidad relacion](public/diagrama_entidad_relacion.png)

### Esquema Principal
La base de datos incluye tablas para:
- **hospitales** - Información de centros médicos
- **users** - Sistema de usuarios y autenticación
- **patients** - Datos de pacientes
- **doctors** - Información de doctores
- **areas, services, specialities** - Catálogos del sistema
- **medical_appointments** - Citas médicas
- **medical_history** - Historial clínico
- **lab_results** - Resultados de laboratorio
- **audit_logs** - Auditoría de cambios

### Script de Base de Datos
Ver archivo `query.sql` para el esquema completo con relaciones y constraints.

## 🔧 Instalación y Configuración

### Prerrequisitos
- **Rust 1.70+** y Cargo
- **PostgreSQL 12+**
- **Git** para clonar el repositorio

### Pasos de Instalación

1. **Clonar el repositorio**
```bash
git clone https://github.com/OrlandoTellez/paciente-app-backend.git
cd paciente-app-backend
```

2. **Configurar variables de entorno**
```bash
cp .env.example .env
# Editar .env con tus configuraciones
```

3. **Configurar base de datos**
```bash
# Ejecutar el script SQL para crear la base de datos
psql -U postgres -f query.sql
```

4. **Instalar dependencias y ejecutar**
```bash
cargo build
cargo run
```

### Variables de Entorno
```env
DATABASE_URL=postgresql://usuario:contraseña@localhost:5432/db_paciente_app
APP_PORT=3000
```

## 📡 API Endpoints

### 🔐 Autenticación (Pendiente de implementación)
*Actualmente los endpoints son públicos para desarrollo*

### 👥 Gestión de Pacientes
- `GET /patients` - Obtener todos los pacientes
- `GET /patients/{id}` - Obtener paciente por ID
- `POST /patients` - Crear nuevo paciente
- `PUT /patients/{id}` - Actualizar paciente
- `DELETE /patients/{id}` - Eliminar paciente (soft delete)

### 🩺 Gestión de Doctores
- `GET /doctors` - Listar doctores
- `GET /doctors/{id}` - Obtener doctor por ID
- `POST /doctors` - Crear nuevo doctor
- `PUT /doctors/{id}` - Actualizar doctor
- `DELETE /doctors/{id}` - Eliminar doctor

### 📅 Citas Médicas
- `GET /appointments` - Listar citas
- `GET /appointments/{id}` - Obtener cita por ID
- `POST /appointments` - Crear nueva cita
- `PATCH /appointments/{id}` - Actualizar cita
- `DELETE /appointments/{id}` - Eliminar cita

### 🏥 Otros Endpoints
- **Usuarios**: `/users` - Gestión de usuarios del sistema
- **Servicios**: `/services` - Catálogo de servicios médicos
- **Especialidades**: `/specialities` - Especialidades médicas
- **Historial Médico**: `/medical_history` - Registros clínicos
- **Resultados de Lab**: `/lab_results` - Exámenes de laboratorio

## ✅ Validación de Datos

El sistema implementa validación robusta en todos los endpoints:

### Validaciones Automáticas
- **Longitud mínima/máxima** en campos de texto
- **Formato de email** válido
- **Números de teléfono** en formato internacional
- **Roles válidos** (patient, doctor, admin)
- **Unicidad** de emails y usernames

### Ejemplo de Validación
```rust
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePatient {
    #[validate(length(min = 2, message = "El nombre debe tener al menos 2 caracteres"))]
    pub first_name: String,
    
    #[validate(email(message = "Email inválido"))]
    pub email: Option<String>,
    
    #[validate(custom = "validate_phone")]
    pub phone: Option<String>,
}
```

## 🔒 Seguridad

### Características Implementadas
- **Hash de contraseñas** con BCrypt
- **Soft delete** para mantener integridad de datos
- **Validación de entrada** para prevenir inyecciones
- **Manejo seguro de errores** sin exposición de información sensible

### Próximas Mejoras de Seguridad
- [ ] Autenticación JWT
- [ ] Autorización por roles
- [ ] Rate limiting
- [ ] CORS configuration

## 🚀 Ejecución y Desarrollo

### Modo Desarrollo
```bash
cargo run
# Servidor disponible en http://localhost:3000
```

### Build de Producción
```bash
cargo build --release
```

### Pruebas
```bash
cargo test
```

## 📊 Estado del Proyecto

### ✅ Completado
- [x] Arquitectura base del proyecto
- [x] CRUD completo para todas las entidades
- [x] Validación de datos robusta
- [x] Configuración con variables de entorno
- [x] Base de datos con relaciones complejas
- [x] Manejo de errores consistente

### 🔄 En Progreso
- [ ] Integración con frontend Tauri
- [ ] Sistema de autenticación JWT
- [ ] Endpoints de reportes y estadísticas

### 📋 Próximas Funcionalidades
- [ ] Sistema de notificaciones
- [ ] Integración con file upload para documentos
- [ ] API para flujo de pacientes en tiempo real
- [ ] Sistema de turnos virtuales
- [ ] Dashboard administrativo

## 🐛 Solución de Problemas

### Problemas Comunes

**Error de conexión a la base de datos:**
```bash
# Verificar que PostgreSQL esté ejecutándose
sudo systemctl status postgresql

# Verificar la cadena de conexión en .env
```

**Error de migraciones:**
```bash
# Ejecutar el script SQL manualmente
psql -U postgres -d db_paciente_app -f query.sql
```

**Problemas de dependencias:**
```bash
# Limpiar y reinstalar
cargo clean
cargo build
```

### Convenciones de Código
- Sigue el estilo de Rust con `rustfmt`
- Usa commits convencionales (feat, fix, docs, etc.)
- Mantén la coherencia con la arquitectura existente

## 📚 Documentación Adicional

- [API Documentation](./documentation/API_CRUD_DOC.md) - Documentación completa de endpoints
- [Database Schema](./documentation/DATABASE_DOC.md) - Esquema detallado de la base de datos
- [Validation System](./documentation/VALIDATION_DATA.md) - Sistema de validación de datos
- [Git Strategy](./documentation/GIT_STRATEGY.md) - Estrategia de control de versiones

## Documentación Rust y Axum

Para soporte técnico o preguntas sobre el proyecto:

- **Documentación de Rust**: [rust-lang.org](https://www.rust-lang.org)
- **Documentación de Axum**: [docs.rs/axum](https://docs.rs/axum)
- **Issues del proyecto**: Crear ticket en el repositorio

---

**Desarrollado con Rust 🦀 para un sistema de salud más eficiente**  
*Tecnología que mejora vidas, código que transforma sistemas*