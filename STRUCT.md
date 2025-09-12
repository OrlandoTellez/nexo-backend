src
├── main.rs               # Punto de entrada
├── config.rs             # Configuración (db, variables de entorno)
├── domain                # Entidades del dominio (hospital, paciente, doctor, etc.)
│   └── hospital.rs
├── application           # Casos de uso (lógica de negocio)
│   └── hospital_service.rs
├── infrastructure        # Adaptadores (db, repositorios, etc.)
│   └── hospital_repository.rs
├── interfaces            # Interfaces/Entrypoints (controllers/handlers HTTP)
│   └── hospital_controller.rs