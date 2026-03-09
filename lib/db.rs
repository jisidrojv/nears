use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

/// Crea el pool de conexiones a PostgreSQL.
/// Se invoca una vez al iniciar la aplicación.
pub async fn crear_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar definida en .env");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error al conectar con PostgreSQL")
}
