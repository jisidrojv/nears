use axum::response::Html;
use axum_extra::extract::cookie::CookieJar;
use maud::html;
use crate::app::layout::layout_raiz;
use crate::components::header::{header, HeaderProps};
use crate::components::footer::footer;
use crate::app::dashboard::components::formulario_contacto::formulario_contacto;
use crate::lib::auth::{no_store_response, require_session};

/// Página para crear contacto — Ruta: /dashboard/crear
pub async fn render(jar: CookieJar) -> axum::response::Response {
    let session = match require_session(&jar) {
        Ok(session) => session,
        Err(response) => return response,
    };

    let props_header = HeaderProps { titulo: "NEARS", mostrar_nav: true, session: Some(session) };

    let pagina = layout_raiz("Crear Contacto | NEARS",
        html! {
            (header(&props_header))
            main class="max-w-[700px] mx-auto px-8 py-10 flex flex-col gap-8" {
                div class="flex items-center gap-4" {
                    a href="/dashboard" class="text-muted-foreground hover:text-foreground transition-colors" {
                        "← Volver"
                    }
                    h1 class="text-2xl font-bold" { "Crear nuevo contacto" }
                }
                
                div class="bg-card border border-border rounded-xl p-6 sm:p-8" {
                    (formulario_contacto(None))
                }
            }
            (footer())
        }
    );

    no_store_response(Html(pagina.into_string()))
}
