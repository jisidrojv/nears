use axum::extract::{Extension, Query};
use axum::http::HeaderMap;
use axum::response::Html;
use axum::Form;
use axum_extra::extract::cookie::CookieJar;
use sqlx::PgPool;
use crate::lib::data::contacto_data::crear_contacto;
use crate::lib::types::contacto_types::ContactoFormulario;
use crate::lib::auth::{create_toast_cookie, no_store_response, require_htmx_session};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContactoQuery {
    pub source: Option<String>,
}

pub async fn post(
    jar: CookieJar,
    Extension(pool): Extension<PgPool>,
    Query(query): Query<ContactoQuery>,
    Form(datos): Form<ContactoFormulario>,
) -> axum::response::Response {
    if query.source.as_deref() == Some("dashboard") {
        if let Err(response) = require_htmx_session(&jar) {
            return response;
        }
    }

    let _contacto = crear_contacto(&pool, &datos).await.unwrap();

    let mut headers = HeaderMap::new();
    
    if query.source.as_deref() == Some("dashboard") {
        let toast_cookie = create_toast_cookie("Registro creado con exito", "success");
        headers.insert("HX-Redirect", "/dashboard".parse().unwrap());
        return no_store_response((jar.add(toast_cookie), headers, Html(String::new())));
    } else {
        headers.insert("HX-Trigger", r#"{"toast":{"message":"Mensaje enviado correctamente","type":"success"}}"#.parse().unwrap());
    }

    no_store_response((headers, Html(String::new())))
}
