use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::FromRow;

/// Registro de envío de formulario de contacto (lectura desde DB).
#[derive(FromRow)]
#[allow(dead_code)]
pub struct ContactoEnvio {
    pub id: i32,
    pub nombre: String,
    pub email: String,
    pub mensaje: String,
    pub activo: bool,
    pub creado: NaiveDateTime,
}

/// Datos del formulario de contacto (escritura desde form HTML).
#[derive(Deserialize)]
pub struct ContactoFormulario {
    pub nombre: String,
    pub email: String,
    pub mensaje: String,
}
