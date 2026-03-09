use sqlx::PgPool;
use crate::lib::types::contacto_types::{ContactoEnvio, ContactoFormulario};

/// Obtiene un contacto por ID.
pub async fn obtener_contacto_por_id(pool: &PgPool, id: i32) -> Option<ContactoEnvio> {
    sqlx::query_as::<_, ContactoEnvio>(
        "SELECT id, nombre, email, mensaje, activo, creado FROM contactos WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .unwrap_or(None)
}

/// Busca contactos con filtro de texto y ordenación.
pub async fn buscar_contactos(pool: &PgPool, busqueda: &str, orden: &str) -> Vec<ContactoEnvio> {
    let order_clause = match orden {
        "antiguo" => "creado ASC",
        "nombre" => "nombre ASC",
        "email" => "email ASC",
        _ => "creado DESC",
    };

    let query = format!(
        "SELECT id, nombre, email, mensaje, activo, creado FROM contactos WHERE nombre ILIKE $1 OR email ILIKE $1 OR mensaje ILIKE $1 ORDER BY {order_clause}"
    );

    let patron = format!("%{busqueda}%");

    sqlx::query_as::<_, ContactoEnvio>(&query)
        .bind(&patron)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
}

/// Cuenta el total de contactos.
pub async fn contar_contactos(pool: &PgPool) -> i64 {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM contactos")
        .fetch_one(pool)
        .await
        .unwrap_or(0)
}

/// Cuenta contactos creados hoy.
pub async fn contar_contactos_hoy(pool: &PgPool) -> i64 {
    sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM contactos WHERE creado >= CURRENT_DATE"
    )
    .fetch_one(pool)
    .await
    .unwrap_or(0)
}

/// Inserta un nuevo contacto.
pub async fn crear_contacto(pool: &PgPool, datos: &ContactoFormulario) -> Result<ContactoEnvio, sqlx::Error> {
    sqlx::query_as::<_, ContactoEnvio>(
        "INSERT INTO contactos (nombre, email, mensaje) VALUES ($1, $2, $3) RETURNING id, nombre, email, mensaje, activo, creado"
    )
    .bind(&datos.nombre)
    .bind(&datos.email)
    .bind(&datos.mensaje)
    .fetch_one(pool)
    .await
}

/// Actualiza un contacto.
pub async fn actualizar_contacto(pool: &PgPool, id: i32, datos: &ContactoFormulario) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE contactos SET nombre = $1, email = $2, mensaje = $3, modificado = NOW() WHERE id = $4")
        .bind(&datos.nombre)
        .bind(&datos.email)
        .bind(&datos.mensaje)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Elimina un contacto.
pub async fn eliminar_contacto(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM contactos WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
