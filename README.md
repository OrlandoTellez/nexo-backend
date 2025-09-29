# Nexo Backend - Documentación 

## Descripción General

Backend para el sistema de gestión de pacientes y citas médicas desarrollado en Rust como parte del hackaton. Este sistema automatiza los procesos de registro, seguimiento de pacientes y programación de citas médicas para hospitales públicos de Nicaragua, desarrollado como solución para el reto avanzado del hackaton.

## Características Principales

### Funcionalidades Implementadas

- **Gestión completa de pacientes**: Operaciones CRUD con validación robusta de datos
- **Sistema de usuarios y autenticación**: Implementación de roles (patient, doctor, admin, admisionist)
- **Gestión de doctores y especialidades médicas**: Catálogo completo de profesionales de la salud
- **Sistema de citas médicas**: Gestión de estados (pending, confirmed, completed, canceled)
- **Historial clínico electrónico**: Registro completo de diagnósticos y tratamientos
- **Resultados de laboratorio**: Gestión de exámenes y pruebas médicas
- **Validación robusta de datos**: Implementada en todos los endpoints
- **Arquitectura hexagonal**: Diseño Domain-Driven Design (DDD)

### Entidades del Sistema

- **Pacientes**: Registro y gestión de información demográfica y médica de pacientes
- **Doctores**: Especialistas médicos con áreas de especialización y servicios
- **Usuarios**: Sistema de autenticación y control de acceso basado en roles
- **Citas Médicas**: Programación, seguimiento y gestión de consultas médicas
- **Historial Médico**: Registro cronológico de diagnósticos, tratamientos y notas médicas
- **Resultados de Laboratorio**: Exámenes, pruebas médicas y sus resultados
- **Servicios y Especialidades**: Catálogos del sistema hospitalario
- **Hospitales**: Gestión de centros médicos y sus datos

## Tecnologías Utilizadas

### Backend Principal
- **Rust**: Lenguaje de programación de sistemas (edición 2021)
- **Axum**: Framework web asíncrono para Rust
- **SQLx**: ORM y cliente de base de datos con soporte para PostgreSQL
- **Tokio**: Runtime asíncrono para aplicaciones de alto rendimiento

### Base de Datos
- **PostgreSQL**: Base de datos relacional principal
- **Características avanzadas**: Soporte para JSONB, foreign keys, transacciones ACID

### Seguridad y Validación
- **BCrypt**: Algoritmo de hash para contraseñas
- **Validator**: Framework de validación de datos con reglas personalizadas
- **Regex**: Validación de formatos específicos (teléfonos, emails)
- **JWT**: Tokens para autenticación y autorización

### Utilidades y Librerías
- **Serde**: Serialización/Deserialización de datos
- **Chrono**: Manejo avanzado de fechas y horas
- **Anyhow**: Manejo simplificado de errores
- **Async-trait**: Soporte para traits asíncronos
- **Tower HTTP**: Middleware para aplicaciones web

## Arquitectura del Proyecto

### Estructura de Directorios

```
src/
├── main.rs                 # Punto de entrada de la aplicación
├── config.rs              # Configuración y variables de entorno
├── domain/                # Capa de dominio (DDD)
│   ├── patient.rs         # Entidad y modelos de paciente
│   ├── doctor.rs          # Entidad y modelos de doctor
│   ├── user.rs            # Entidad y modelos de usuario
│   ├── appointment.rs     # Entidad y modelos de cita médica
│   ├── medical_history.rs # Modelos de historial médico
│   ├── lab_result.rs      # Modelos de resultados de laboratorio
│   ├── auth.rs            # Modelos de autenticación
│   ├── hospital.rs        # Modelos de hospital
│   ├── services.rs        # Modelos de servicios médicos
│   └── speciality.rs      # Modelos de especialidades
├── application/           # Capa de aplicación (casos de uso)
│   ├── patient_service.rs
│   ├── doctor_service.rs
│   ├── user_service.rs
│   ├── hospital_service.rs
│   ├── services_service.rs
│   ├── speciality_service.rs
│   ├── appointment_service.rs
│   ├── medical_history_service.rs
│   ├── lab_result_service.rs
│   └── auth_service.rs
├── infrastructure/        # Capa de infraestructura
│   ├── patient_repository.rs
│   ├── doctor_repository.rs
│   ├── user_repository.rs
│   ├── hospital_repository.rs
│   ├── services_repository.rs
│   ├── speciality_repository.rs
│   ├── appointment_repository.rs
│   ├── medical_history_repository.rs
│   ├── lab_result_repository.rs
│   └── auth_repository.rs
├── interfaces/           # Controladores HTTP (entrypoints)
│   ├── patient_controller.rs
│   ├── doctor_controller.rs
│   ├── user_controller.rs
│   ├── hospital_controller.rs
│   ├── services_controller.rs
│   ├── speciality_controller.rs
│   ├── appointment_controller.rs
│   ├── medical_history_controller.rs
│   ├── lab_result_controller.rs
│   └── auth_controller.rs
├── routes/               # Definición de rutas de la API
│   ├── patient.rs
│   ├── doctor.rs
│   ├── user.rs
│   ├── hospital.rs
│   ├── services.rs
│   ├── speciality.rs
│   ├── appointment.rs
│   ├── medical_history.rs
│   ├── lab_result.rs
│   └── auth.rs
└── helpers/              # Utilidades y helpers
    ├── validators.rs     # Validadores personalizados
    ├── jwt.rs           # Utilidades JWT
    └── utils.rs         # Funciones utilitarias
```

### Patrones de Diseño Implementados

- **Arquitectura Hexagonal**: Separación clara entre dominio, aplicación e infraestructura
- **Repository Pattern**: Abstracción del acceso a datos
- **Service Layer**: Lógica de negocio centralizada
- **DTO Pattern**: Transferencia de datos entre capas
- **Dependency Injection**: Inversión de control para testing y mantenibilidad

## Base de Datos

### Diagrama de Entidad-Relación

![Diagrama Entidad-Relación](public/diagrama_entidad_relacion_v2.png)

### Esquema Principal

La base de datos incluye las siguientes tablas principales:

- **hospitals**: Información de centros médicos
- **users**: Sistema de usuarios y autenticación
- **patients**: Datos demográficos y médicos de pacientes
- **doctors**: Información profesional de doctores
- **areas, services, specialities**: Catálogos del sistema hospitalario
- **medical_appointments**: Citas médicas con estados y metadata
- **medical_history**: Historial clínico electrónico
- **lab_results**: Resultados de exámenes de laboratorio
- **audit_logs**: Auditoría de cambios en el sistema
- **notifications**: Sistema de notificaciones
- **medical_documents**: Gestión de documentos médicos
- **virtual_turns**: Sistema de turnos virtuales

### Características del Esquema

- **Soft Delete**: Eliminación lógica mediante campos `deleted_at`
- **Auditoría**: Registro de cambios críticos en `audit_logs`
- **Índices Optimizados**: Para consultas frecuentes
- **Restricciones de Integridad**: Foreign keys y constraints de validación
- **Campos de Metadata**: `created_at`, `updated_at` para trazabilidad

## Instalación y Configuración

### Prerrequisitos

- **Rust 1.70+** y Cargo
- **PostgreSQL 12+**
- **Git** para control de versiones

### Configuración del Entorno

1. **Clonar el repositorio**
```bash
git clone https://github.com/OrlandoTellez/nexo-backend.git
cd nexo-backend
```

2. **Configurar variables de entorno**
```bash
cp .env.example .env
# Editar .env con configuraciones específicas
```

3. **Configurar base de datos**
```bash
# Crear y poblar la base de datos
psql -U postgres -f query.sql
```

4. **Instalar dependencias y ejecutar**
```bash
cargo build
cargo run
```

### Variables de Entorno Requeridas

```env
DATABASE_URL=postgresql://usuario:contraseña@localhost:5432/db_paciente_app
APP_PORT=3000
FRONTEND_URL=http://localhost:3000
JWT_SECRET=tu_jwt_secret_muy_seguro
```

## API Endpoints

### Autenticación

- `POST /auth/login` - Iniciar sesión
- `POST /auth/logout` - Cerrar sesión

### Gestión de Pacientes

- `GET /patients` - Listar todos los pacientes
- `GET /patients/{id}` - Obtener paciente específico
- `POST /patients` - Crear nuevo paciente
- `PATCH /patients/{id}` - Actualizar paciente
- `DELETE /patients/{id}` - Eliminar paciente (soft delete)

### Gestión de Doctores

- `GET /doctors` - Listar doctores
- `GET /doctors/{id}` - Obtener doctor específico
- `POST /doctors` - Crear nuevo doctor
- `PATCH /doctors/{id}` - Actualizar doctor
- `DELETE /doctors/{id}` - Eliminar doctor

### Citas Médicas

- `GET /appointments` - Listar citas
- `GET /appointments/{id}` - Obtener cita específica
- `POST /appointments` - Crear nueva cita
- `PATCH /appointments/{id}` - Actualizar cita
- `DELETE /appointments/{id}` - Eliminar cita

### Catálogos del Sistema

- **Hospitales**: `/hospitales` - Gestión de centros médicos
- **Servicios**: `/services` - Catálogo de servicios médicos
- **Especialidades**: `/specialities` - Especialidades médicas
- **Áreas**: `/areas` - Áreas médicas del hospital

### Registros Médicos

- **Historial Médico**: `/medical_history` - Registros clínicos
- **Resultados de Laboratorio**: `/lab_results` - Exámenes médicos

## Validación de Datos

### Sistema de Validación

El sistema implementa validación robusta en múltiples niveles:

1. **Validación de Esquema**: Usando el trait `Validate` de la librería `validator`
2. **Validación de Negocio**: En la capa de servicio
3. **Validación de Base de Datos**: Constraints a nivel de BD

### Ejemplos de Validación

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePatient {
    #[validate(length(min = 2, message = "El nombre debe tener al menos 2 caracteres"))]
    pub first_name: String,
    
    #[validate(email(message = "Email inválido"))]
    pub email: Option<String>,
    
    #[validate(custom = "validate_phone")]
    pub phone: Option<String>,
    
    #[validate(length(min = 5, message = "La cédula debe tener al menos 5 caracteres"))]
    pub identity_number: String,
}
```

### Validadores Personalizados

- **Teléfonos**: Formato internacional E.164
- **Roles**: Valores permitidos (patient, doctor, admin, admisionist)
- **Tipos de Sangre**: Valores estándar (A+, O-, etc.)
- **Géneros**: M, F, O

## Seguridad

### Características Implementadas

- **Hash de Contraseñas**: BCrypt con salt automático
- **Soft Delete**: Mantenimiento de integridad referencial
- **Validación de Entrada**: Prevención de inyecciones
- **Manejo Seguro de Errores**: Sin exposición de información sensible
- **CORS Configurado**: Control de origenes permitidos

### Autenticación JWT

```rust
pub fn generate_jwt(sub: String, role: String) -> Result<String> {
    let secret = env::var("JWT_SECRET")?;
    let claims = Claims::new(sub, role, 60); // 60 minutos
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
```

### Próximas Mejoras de Seguridad

- [ ] Rate limiting por IP y usuario
- [ ] Validación de contenido malicioso
- [ ] Auditoría de seguridad completa
- [ ] Encriptación de datos sensibles en reposo

## Ejecución y Desarrollo

### Modos de Ejecución

**Desarrollo:**
```bash
cargo run
# Servidor disponible en http://localhost:3000
```

**Producción:**
```bash
cargo build --release
./target/release/paciente-app-backend
```

**Testing:**
```bash
cargo test
```

### Variables de Configuración por Entorno

```env
# Desarrollo
DATABASE_URL=postgresql://user:pass@localhost:5432/db_paciente_app_dev
APP_PORT=3000
RUST_LOG=debug

# Producción
DATABASE_URL=postgresql://user:pass@prod-server:5432/db_paciente_app
APP_PORT=8080
RUST_LOG=info
```

## Estado del Proyecto

### Completado

- [x] Arquitectura base del proyecto (DDD/Hexagonal)
- [x] CRUD completo para todas las entidades principales
- [x] Sistema de validación robusto
- [x] Configuración con variables de entorno
- [x] Base de datos con relaciones complejas
- [x] Manejo de errores consistente
- [x] Sistema de autenticación JWT

### En Desarrollo

- [ ] Integración completa con frontend Tauri
- [ ] Sistema de notificaciones en tiempo real
- [ ] Endpoints de reportes y estadísticas
- [ ] Optimización de consultas complejas

### Próximas Funcionalidades

- [ ] Sistema de turnos virtuales
- [ ] Integración con file upload para documentos
- [ ] API para flujo de pacientes en tiempo real
- [ ] Dashboard administrativo
- [ ] Integración con sistemas externos de salud

## Solución de Problemas

### Problemas Comunes y Soluciones

**Error de Conexión a Base de Datos:**
```bash
# Verificar estado de PostgreSQL
sudo systemctl status postgresql

# Verificar cadena de conexión en .env
echo $DATABASE_URL
```

**Error de Migraciones:**
```bash
# Ejecutar script manualmente
psql -U postgres -d db_paciente_app -f query.sql
```

**Problemas de Dependencias:**
```bash
# Limpiar cache y reinstalar
cargo clean
cargo build
```

**Problemas de Puerto:**
```bash
# Verificar puerto disponible
sudo lsof -i :3000

# Cambiar puerto en .env
APP_PORT=3001
```

### Convenciones de Código

- **Formato**: Seguir `rustfmt` para estilo consistente
- **Commits**: Usar conventional commits (feat, fix, docs, etc.)
- **Documentación**: Mantener documentación actualizada
- **Testing**: Escribir tests para nueva funcionalidad

## Documentación Adicional

- [API Documentation](./documentation/API_CRUD_DOC.md) - Documentación completa de endpoints
- [Database Schema](./documentation/DATABASE_DOC.md) - Esquema detallado de la base de datos
- [Validation System](./documentation/VALIDATION_DATA.md) - Sistema de validación de datos
- [Git Strategy](./documentation/GIT_STRATEGY.md) - Estrategia de control de versiones

## Soporte Técnico

Para soporte técnico o preguntas sobre el proyecto:

- **Documentación de Rust**: [rust-lang.org](https://www.rust-lang.org)
- **Documentación de Axum**: [docs.rs/axum](https://docs.rs/axum)
- **SQLx Documentation**: [docs.rs/sqlx](https://docs.rs/sqlx)
- **Issues del Proyecto**: Crear ticket en el repositorio GitHub

---

**Desarrollado con Rust para un sistema de salud más eficiente**  
*Tecnología que mejora vidas, código que transforma sistemas*
