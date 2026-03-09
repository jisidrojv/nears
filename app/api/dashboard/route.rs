use axum::extract::{Extension, Query};
use axum::response::Html;
use axum_extra::extract::cookie::CookieJar;
use sqlx::PgPool;
use crate::lib::data::contacto_data::{buscar_contactos, contar_contactos, contar_contactos_hoy};
use crate::app::dashboard::page::{contenido_dashboard, FiltrosDashboard};
use crate::lib::auth::{no_store_response, require_htmx_session};

pub async fn get(
    jar: CookieJar,
    Extension(pool): Extension<PgPool>,
    Query(filtros): Query<FiltrosDashboard>,
) -> axum::response::Response {
    if let Err(response) = require_htmx_session(&jar) {
        return response;
    }

    let busqueda = filtros.busqueda.unwrap_or_default();
    let orden = filtros.orden.unwrap_or_else(|| "reciente".to_string());
    let contactos = buscar_contactos(&pool, &busqueda, &orden).await;
    let total = contar_contactos(&pool).await;
    let hoy = contar_contactos_hoy(&pool).await;

    no_store_response(Html(contenido_dashboard(&contactos, total, hoy, &busqueda, &orden).into_string()))
}
