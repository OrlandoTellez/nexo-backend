# Validación de Datos en Paciente App backend

## Descripción

La validación de datos es un mecanismo que asegura **la integridad y corrección de la información ingresada por los usuarios** antes de ser procesada o almacenada en la base de datos. Esto es crucial para:

* Evitar datos inconsistentes o incompletos.
* Prevenir errores en la aplicación o en la base de datos.
* Mejorar la experiencia del usuario mediante retroalimentación inmediata de errores.

En este proyecto, la validación se implementa en **todos los puntos de entrada de datos**, principalmente en los endpoints que crean o actualizan registros de usuarios, pacientes, doctores, hospitales y otros elementos del sistema.

---

## Librerías Utilizadas

* [`validator`](https://docs.rs/validator/latest/validator/): Permite definir reglas de validación sobre los structs de datos usando atributos.
* [`regex`](https://docs.rs/regex/latest/regex/): Se utiliza para validar formatos específicos, como teléfonos.
* [`lazy_static`](https://docs.rs/lazy_static/latest/lazy_static/): Permite inicializar expresiones regulares solo una vez para optimizar el rendimiento.

---

## Cómo Funciona la Validación

### 1. Validación en los Structs

Los datos que provienen del usuario se representan mediante structs de Rust (`CreateUser`, `UpdateUser`, `CreatePatient`, `UpdatePatient`, etc.). Estos structs implementan el trait `Validate` de la librería `validator`:

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, message = "El username debe tener al menos 3 caracteres"))]
    pub username: String,

    #[validate(length(min = 6, message = "La contraseña debe tener al menos 6 caracteres"))]
    pub password_hash: String,

    #[validate(custom = "validate_role")]
    pub role: String,
}
```

* `length(min = N)`: Asegura que un texto tenga al menos N caracteres.
* `email`: Verifica que el texto sea un correo válido.
* `custom`: Permite definir funciones de validación propias.

---

### 2. Funciones de Validación Personalizadas

En `src/helpers/validators.rs` se definen validaciones específicas:

```rust
pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    lazy_static::lazy_static! {
        static ref PHONE_REGEX: Regex = Regex::new(r"^\+[1-9][0-9]{7,14}$").unwrap();
    }
    if PHONE_REGEX.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationError::new("phone"))
    }
}

pub fn validate_role(role: &str) -> Result<(), ValidationError> {
    match role {
        "patient" | "doctor" | "admin" => Ok(()),
        _ => Err(ValidationError::new("role")),
    }
}
```

* `validate_phone`: Verifica que el número cumpla con el formato internacional **E.164** (`+50575061202`).
* `validate_role`: Asegura que el rol sea uno de los permitidos (`patient`, `doctor`, `admin`).

Para campos opcionales (`Option<String>`), se valida solo si el valor está presente:

```rust
pub fn validate_phone_opt(phone: &Option<String>) -> Result<(), ValidationError> {
    if let Some(ref p) = phone {
        validate_phone(p)?;
    }
    Ok(())
}
```

---

### 3. Validación en los Controladores

Antes de procesar los datos, los controladores llaman a la función `validate()`:

```rust
if let Err(errors) = payload.validate() {
    return (
        StatusCode::BAD_REQUEST,
        Json(format!("Errores de validación: {:?}", errors)),
    ).into_response();
}
```

* Si hay errores, se devuelve **400 Bad Request** con un mensaje detallando los problemas.
* Si los datos son válidos, se procede a procesarlos (por ejemplo, hashear contraseñas o guardar en la base de datos).

---

## Flujo de Validación

1. **El usuario envía datos** a la API (JSON).
2. **El controlador recibe el struct** (`CreateX` o `UpdateX`).
3. Se llama a `validate()`.
4. **Si hay errores** → se devuelve un mensaje de error y el proceso se detiene.
5. **Si es válido** → se continúa con la lógica de negocio (guardar, actualizar, etc.).

---

## Ventajas de este enfoque

* Garantiza **integridad y consistencia** de la información.
* Los errores son claros y fáciles de identificar.
* Compatible con campos opcionales y reglas personalizadas.
* Previene problemas en la base de datos y errores de la aplicación.



