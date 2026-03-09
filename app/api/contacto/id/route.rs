use axum::extract::{Extension, Path};
use axum::http::HeaderMap;
use axum::response::Html;
use axum::Form;
use axum_extra::extract::cookie::CookieJar;
use sqlx::PgPool;
use crate::lib::data::contacto_data::{actualizar_contacto, eliminar_contacto, buscar_contactos, contar_contactos, contar_contactos_hoy};
use crate::lib::types::contacto_types::ContactoFormulario;
use crate::app::dashboard::page::contenido_dashboard;
use crate::lib::auth::{create_toast_cookie, no_store_response, require_htmx_session};

pub async fn put(
    jar: CookieJar,
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Form(datos): Form<ContactoFormulario>,
) -> axum::response::Response {
    if let Err(response) = require_htmx_session(&jar) {
        return response;
    }

    let _ = actualizar_contacto(&pool, id, &datos).await;
    let toast_cookie = create_toast_cookie("Registro actualizado", "success");

    let mut headers = HeaderMap::new();
    headers.insert("HX-Redirect", format!("/dashboard/{}", id).parse().unwrap());

    no_store_response((jar.add(toast_cookie), headers, Html(String::new())))
}

pub async fn delete(
    jar: CookieJar,
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> axum::response::Response {
    if let Err(response) = require_htmx_session(&jar) {
        return response;
    }

    let _ = eliminar_contacto(&pool, id).await;
    let contactos = buscar_contactos(&pool, "", "reciente").await;
    let total = contar_contactos(&pool).await;
    let hoy = contar_contactos_hoy(&pool).await;

    let mut headers = HeaderMap::new();
    headers.insert("HX-Trigger", r#"{"toast":{"message":"Registro eliminado","type":"success"}}"#.parse().unwrap());

    no_store_response((headers, Html(contenido_dashboard(&contactos, total, hoy, "", "reciente").into_string())))
}
