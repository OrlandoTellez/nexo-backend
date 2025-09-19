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