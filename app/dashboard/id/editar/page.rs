use axum::extract::{Extension, Path};
use axum::response::Html;
use axum_extra::extract::cookie::CookieJar;
use maud::html;
use sqlx::PgPool;
use crate::app::layout::layout_raiz;
use crate::components::header::{header, HeaderProps};
use crate::components::footer::footer;
use crate::app::dashboard::components::formulario_contacto::formulario_contacto;
use crate::lib::data::contacto_data::obtener_contacto_por_id;
use crate::lib::auth::{no_store_response, require_session};

/// Página para editar contacto — Ruta: /dashboard/{id}/editar
pub async fn render(
    jar: CookieJar,
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> axum::response::Response {
    let session = match require_session(&jar) {
        Ok(session) => session,
        Err(response) => return response,
    };

    let props_header = HeaderProps { titulo: "NEARS", mostrar_nav: true, session: Some(session) };

    let contacto_opt = obtener_contacto_por_id(&pool, id).await;

    let pagina = layout_raiz("Editar Contacto | NEARS",
        html! {
            (header(&props_header))
            main class="max-w-[700px] mx-auto px-8 py-10 flex flex-col gap-8" {
                div class="flex items-center gap-4" {
                    a href={"/dashboard/" (id)} class="text-muted-foreground hover:text-foreground transition-colors" {
                        "← Volver"
                    }
                    h1 class="text-2xl font-bold max-w-sm truncate" { 
                        "Editar " 
                        @if let Some(c) = &contacto_opt { (c.nombre) } @else { "contacto" }
                    }
                }

                div class="bg-card border border-border rounded-xl p-6 sm:p-8" {
                    @if let Some(contacto) = contacto_opt {
                        (formulario_contacto(Some(&contacto)))
                    } @else {
                        div class="text-center py-12" {
                            p class="text-destructive font-medium" { "Registro no encontrado." }
                        }
                    }
                }
            }
            (footer())
        }
    );

    no_store_response(Html(pagina.into_string()))
}
