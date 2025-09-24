src
├── main.rs               # Punto de entrada
├── config.rs             # Configuración (db, variables de entorno)
├── domain                # Entidades del dominio (hospital, paciente, doctor, etc.)
│   └── paciente.rs
├── application           # Casos de uso (lógica de negocio)
│   └── pacient_service.rs
├── infrastructure        # Adaptadores (db, repositorios, etc.)
│   └── paciente_repository.rs
├── interfaces            # Interfaces/Entrypoints (controllers/handlers HTTP)
│   └── paciente_controller.rs
├── routes                # Rutas de la aplicación
│   └── paciente.rs