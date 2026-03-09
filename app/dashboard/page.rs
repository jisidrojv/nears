use axum::extract::{Extension, Query};
use axum_extra::extract::cookie::CookieJar;
use axum::response::Html;
use maud::html;
use sqlx::PgPool;
use serde::Deserialize;

use crate::app::layout::layout_raiz;
use crate::components::header::{header, HeaderProps};
use crate::components::footer::footer;
use crate::components::ui::dialog_confirmation::{dialog_confirmacion, ConfirmacionVariante};
use crate::app::dashboard::components::card_stat::card_stat;
use crate::app::dashboard::components::barra_herramientas::barra_herramientas;
use crate::app::dashboard::components::tabla::renderizar_tabla;
use crate::lib::data::contacto_data::{buscar_contactos, contar_contactos, contar_contactos_hoy};
use crate::lib::auth::{no_store_response, require_session};

#[derive(Deserialize)]
pub struct FiltrosDashboard {
    pub busqueda: Option<String>,
    pub orden: Option<String>,
}

/// Página de dashboard — Ruta: /dashboard
pub async fn render(
    jar: CookieJar,
    Extension(pool): Extension<PgPool>,
    Query(filtros): Query<FiltrosDashboard>,
) -> axum::response::Response {
    // 1. Verificación de sesión (Patrón DAL)
    let _session = match require_session(&jar) {
        Ok(session) => session,
        Err(response) => return response,
    };
    let busqueda = filtros.busqueda.unwrap_or_default();
    let orden = filtros.orden.unwrap_or_else(|| "reciente".to_string());
    let contactos = buscar_contactos(&pool, &busqueda, &orden).await;
    let total = contar_contactos(&pool).await;
    let hoy = contar_contactos_hoy(&pool).await;

    let props_header = HeaderProps { titulo: "NEARS", mostrar_nav: true, session: Some(_session) };

    let pagina = layout_raiz("Dashboard | NEARS",
        html! {
            (header(&props_header))
            main class="max-w-[1100px] mx-auto px-8 py-10 flex flex-col gap-8" {
                h1 class="text-2xl font-bold" { "Dashboard" }
                div id="dashboard-contenido" {
                    (contenido_dashboard(&contactos, total, hoy, &busqueda, &orden))
                }
            }
            (footer())
            (dialog_confirmacion("dialog-eliminar", "Eliminar registro", "Esta acción no se puede deshacer.", ConfirmacionVariante::Eliminar))
        }
    );

    no_store_response(Html(pagina.into_string()))
}

/// Fragmento interior del dashboard (reutilizable por HTMX).
pub fn contenido_dashboard(
    contactos: &[crate::lib::types::contacto_types::ContactoEnvio],
    total: i64, hoy: i64, busqueda: &str, orden: &str,
) -> maud::Markup {
    html! {
        // Stats
        div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4" {
            (card_stat("📊", "Total registros", &total.to_string(), None))
            (card_stat("📅", "Contactos hoy", &hoy.to_string(), Some("Últimas 24 horas")))
            (card_stat("📋", "Mostrando", &contactos.len().to_string(), Some("Resultados filtrados")))
        }
        // Herramientas
        div class="pt-2" {
            (barra_herramientas(busqueda, orden))
        }
        // Tabla
        div id="tabla-contactos" class="pt-2" {
            (renderizar_tabla(contactos))
        }
    }
}
