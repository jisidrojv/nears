#![allow(special_module_name)]

mod app;
mod components;
mod lib;

use axum::{Router, Extension};
use tower_http::services::{ServeDir, ServeFile};

mod nears_routes {
    include!(concat!(env!("OUT_DIR"), "/nears_routes_generated.rs"));
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let direccion = format!("{host}:{port}");

    let pool = lib::db::crear_pool().await;
    println!("🐘 PostgreSQL conectado");

    let app = Router::new()
        .merge(nears_routes::auto_router())
        .route_service("/static/style.css", ServeFile::new("public/style.css"))
        .nest_service("/static", ServeDir::new("public"))
        .layer(Extension(pool));

    println!("🦀 NEARS corriendo en http://{direccion}");

    let listener = tokio::net::TcpListener::bind(&direccion).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
