endpoints para pacientes:

obtener todos los pacientes
GET /patients
obtener un paciente por id
GET /patients/{id}
crear un nuevo paciente
POST /patients
actualizar un paciente
PUT /patients/{id}
eliminar un paciente
DELETE /patients/{id}

ejemplo de use 
POST /patients
body:
{
  "id_user": null, // por defecto es null
  "first_name": "Samuel",
  "second_name": "Gabriel",
  "first_lastname": "Tellez",
  "second_lastname": "Houston",
  "address": "Rpto. satelite asososca casa no  135",
  "birthdate": "2005-01-06",
  "phone": "75061202",
  "email": "orlandotellsez36@gmail.com"
}

endpoints para usuarios:

obtener todos los usuarios
GET /users
obtener un usuario por id
GET /users/{id}
crear un nuevo usuario
POST /users
actualizar un usuario
PUT /users/{id}
eliminar un usuario
DELETE /users/{id}

ejemplo de use 
POST /users
body:
{
  "username": "orlandotellsez36",
  "password_hash": "$2a$10$5.9.1.0.3.2.5.4.6.7.8.9.1.2.3.4.5.6.7.8.9.1",
  "role": "patient"
}

endpoints para doctores:

obtener todos los doctores
GET /doctores
obtener un doctor por id
GET /doctores/{id}
crear un nuevo doctor
POST /doctores
actualizar un doctor
PUT /doctores/{id}
eliminar un doctor
DELETE /doctores/{id}

ejemplo de use 
POST /doctores

// primero se necesita tener servicios, areas y especialidades creados
body:
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


endpoints para servicios:

obtener todos los servicios
GET /services
obtener un servicio por id
GET /services/{id}
crear un nuevo servicio
POST /services
actualizar un servicio
PATCH /services/{id}
eliminar un servicio
DELETE /services/{id}

ejemplo de use 
POST /services
body:
{
  "service_name": "Servicio 1"
}