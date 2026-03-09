use axum::extract::{Extension, Path};
use axum::response::Html;
use axum_extra::extract::cookie::CookieJar;
use maud::html;
use sqlx::PgPool;
use crate::app::layout::layout_raiz;
use crate::components::header::{header, HeaderProps};
use crate::components::footer::footer;
use crate::lib::data::contacto_data::obtener_contacto_por_id;
use crate::lib::auth::{no_store_response, require_session};

/// Página de detalle de contacto — Ruta: /dashboard/{id}
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

    let pagina = layout_raiz("Detalle de Contacto | NEARS",
        html! {
            (header(&props_header))
            main class="max-w-[700px] mx-auto px-8 py-10 flex flex-col gap-8" {
                div class="flex items-center justify-between" {
                    div class="flex items-center gap-4" {
                        a href="/dashboard" class="text-muted-foreground hover:text-foreground transition-colors" {
                            "← Dashboard"
                        }
                        h1 class="text-2xl font-bold" { "Detalle de contacto" }
                    }
                    
                    @if contacto_opt.is_some() {
                        a href={"/dashboard/" (id) "/editar"}
                           class="inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg text-sm font-medium bg-secondary text-secondary-foreground hover:bg-secondary/80 border border-border" {
                            "✏️ Editar"
                       }
                    }
                }

                div class="bg-card border border-border rounded-xl p-6 sm:p-8 flex flex-col gap-6" {
                    @if let Some(contacto) = contacto_opt {
                        div class="flex flex-col gap-1.5" {
                            span class="text-sm font-medium text-muted-foreground" { "ID" }
                            span class="text-base text-foreground font-mono" { (contacto.id) }
                        }
                        div class="flex flex-col gap-1.5" {
                            span class="text-sm font-medium text-muted-foreground" { "Nombre" }
                            span class="text-base text-foreground font-medium" { (contacto.nombre) }
                        }
                        div class="flex flex-col gap-1.5" {
                            span class="text-sm font-medium text-muted-foreground" { "Email" }
                            a href={"mailto:" (contacto.email)} class="text-base text-primary hover:underline hover:text-primary/80 transition-colors" { (contacto.email) }
                        }
                        div class="flex flex-col gap-1.5" {
                            span class="text-sm font-medium text-muted-foreground" { "Mensaje" }
                            div class="text-base text-foreground bg-accent/50 p-4 rounded-md whitespace-pre-wrap border border-border/50" { 
                                (contacto.mensaje)
                            }
                        }
                        div class="flex flex-col gap-1.5 pt-4 border-t border-border mt-2" {
                            span class="text-sm font-medium text-muted-foreground" { "Fecha de creación" }
                            span class="text-sm text-foreground" { (contacto.creado.format("%d/%m/%Y %H:%M")) }
                        }
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
