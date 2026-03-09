// Script: seed.rs
// Simula `prisma db seed`. Inserta los roles y el usuario admin.

use sqlx::postgres::PgPoolOptions;
use std::env;
use bcrypt::{hash, DEFAULT_COST};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    println!("🌱 Iniciando seeding de la base de datos...\n");

    let database_url = env::var("DATABASE_URL").expect("Falta DATABASE_URL en .env");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Ejecutar migraciones pendientes automáticamente usando sqlx::migrate!
    println!("⚙️ Ejecutando migraciones...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("✅ Migraciones completadas.");

    // ------------ 1. SEED ROLES ------------
    println!("📝 Seeding roles...");
    let roles_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM roles")
        .fetch_one(&pool)
        .await?;

    if roles_count.0 > 0 {
        println!("   ⏭️  Roles ya existen, saltando...");
    } else {
        sqlx::query("INSERT INTO roles (nombre, es_admin) VALUES ('admin', true)")
            .execute(&pool).await?;
        println!("   ✅ Rol creado: admin");

        sqlx::query("INSERT INTO roles (nombre, es_admin) VALUES ('usuario', false)")
            .execute(&pool).await?;
        println!("   ✅ Rol creado: usuario");
    }

    // ------------ 2. SEED USUARIOS ------------
    println!("\n📝 Seeding usuarios...");
    let admin_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM usuarios WHERE email = 'admin@nears.dev'")
        .fetch_one(&pool)
        .await?;

    if admin_count.0 > 0 {
        println!("   ⏭️  Usuario admin ya existe, saltando...");
    } else {
        let (admin_rol_id,): (i32,) = sqlx::query_as("SELECT id FROM roles WHERE nombre = 'admin'")
            .fetch_one(&pool)
            .await?;

        let pass_hash = hash("nears/admin!", DEFAULT_COST)?;

        sqlx::query(
            "INSERT INTO usuarios (email, password_hash, nombre, apellidos, imagen, rol_id, activo)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind("admin@nears.dev")
        .bind(&pass_hash)
        .bind("Nears")
        .bind("Admin")
        .bind("/nears/nears-art.webp")
        .bind(admin_rol_id)
        .bind(true)
        .execute(&pool).await?;

        println!("   ✅ Usuario admin creado: admin@nears.dev");
        println!("   🔑 Password: nears/admin!");
        println!("   ⚠️  IMPORTANTE: Cambia esta contraseña después del primer login\n");
    }

    println!("\n🎉 Seeding completado con éxito!");
    Ok(())
}
