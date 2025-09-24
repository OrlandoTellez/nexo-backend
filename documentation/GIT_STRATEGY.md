
````markdown
#  Flujo de trabajo con Git y GitHub

Este proyecto se gestionó utilizando **Git** y repositorios en **GitHub**.  
El equipo trabajó con un flujo de ramas para mantener orden y control en el desarrollo.

---

##  Estructura de ramas

- **`master`**  
  - Rama principal y estable.  
  - Contiene solo las versiones listas para producción.  

- **`develop`**  
  - Rama de integración para desarrollo y pruebas.  
  - Aquí se fusionan todas las ramas `feature/*` antes de pasar a `master`.  

- **`feature/*`**  
  - Ramas específicas para nuevas funcionalidades o mejoras.  
  - Ejemplo:  
    - `feature/login`  
    - `feature/crud-pacientes`  
    - `feature/notificaciones`  

---

## Flujo de trabajo

1. **Crear rama de feature desde `develop`:**
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feature/nombre-funcionalidad
````

2. **Desarrollar la funcionalidad y hacer commits descriptivos:**

   ```bash
   git add .
   git commit -m "feat: agregar validación de citas médicas"
   ```

3. **Subir la rama al repositorio remoto:**

   ```bash
   git push origin feature/nombre-funcionalidad
   ```

4. **Crear un Pull Request (PR) en GitHub hacia `develop`.**

5. **Revisar el código en equipo**

   * Comentarios y sugerencias en el PR.
   * Si todo está correcto, se aprueba y se hace merge en `develop`.

6. **Pruebas en `develop`**

   * Una vez que la rama `develop` está estable y probada, se fusiona en `master`.
7. **Eliminar la rama de feature.**
   * Eliminar la rama de feature en GitHub.
   * Eliminar la rama de feature en el repositorio local.

---

##  Ejemplo de flujo

* `feature/login` → merge en `develop`
* `feature/citas` → merge en `develop`
* `develop` (después de pruebas) → merge en `master`

---

##  Convenciones de commits

Se usó el formato [Conventional Commits](https://www.conventionalcommits.org/):

* `feat:` → nueva funcionalidad
* `fix:` → corrección de errores
* `docs:` → cambios en documentación
* `refactor:` → mejoras internas sin cambiar funcionalidad
* `style:` → cambios de estilo o formato de código
* `test:` → pruebas añadidas o modificadas

Ejemplo:

```bash
git commit -m "feat: implementación del CRUD de pacientes"
```

---

##  Beneficios de este flujo

* Mantiene **controlado el código** en producción.
* Permite **trabajo en paralelo** con ramas de funcionalidades.
* Facilita **revisiones en equipo** mediante PRs.
* Reduce conflictos y asegura que `master` siempre esté **estable**.

---

