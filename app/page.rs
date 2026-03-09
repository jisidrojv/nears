use axum::response::Html;
use maud::html;
use crate::app::layout::layout_raiz;
use crate::components::header::{header, HeaderProps};
use crate::components::footer::footer;

/// Página de inicio — Ruta: /
pub async fn render() -> Html<String> {
    let props_header = HeaderProps {
        titulo: "NEARS",
        mostrar_nav: true,
        session: None,
    };

    let pagina = layout_raiz("NEARS — Framework Web Nativo en Rust",
        html! {
            (header(&props_header))

            main {
                // Hero
                section class="relative overflow-hidden px-8 py-16 text-center" {
                    // Glow decorativo
                    div class="absolute top-[-50%] left-1/2 -translate-x-1/2 w-[600px] h-[600px] bg-[radial-gradient(circle,var(--nears-glow)_0%,transparent_70%)] pointer-events-none" {}

                    div class="relative max-w-[720px] mx-auto flex flex-col items-center gap-6" {
                        span class="inline-flex items-center gap-2 px-4 py-1 bg-neaxt-primary/10 border border-neaxt-primary/20 rounded-full text-sm font-medium text-neaxt-primary animate-fade-in-down" {
                            "🦀 Rust Nativo"
                        }
                        h1 class="text-4xl md:text-5xl lg:text-6xl font-extrabold tracking-tight animate-fade-in-up animation-delay-100" {
                            "Construye aplicaciones web"
                            br;
                            span class="bg-gradient-to-r from-neaxt-primary to-neaxt-secondary bg-clip-text text-transparent" {
                                "ultrarrápidas y seguras"
                            }
                        }
                        p class="text-lg text-muted-foreground max-w-[560px] animate-fade-in-up animation-delay-200" {
                            "Framework web con opinión fuerte. Estructura familiar de Next.js, "
                            "potencia de Rust, cero JavaScript obligatorio."
                        }
                        div class="flex gap-4 animate-fade-in-up animation-delay-300" {
                            a href="/dashboard"
                              class="inline-flex items-center justify-center px-8 py-2.5 rounded-md text-sm font-semibold bg-gradient-to-r from-neaxt-primary to-neaxt-secondary text-primary-foreground shadow-lg shadow-neaxt-primary/20 hover:-translate-y-0.5 hover:shadow-xl hover:shadow-neaxt-primary/30" {
                                "Ver Dashboard"
                            }
                            a href="/contacto"
                              class="inline-flex items-center justify-center px-8 py-2.5 rounded-md text-sm font-semibold bg-accent text-accent-foreground border border-border hover:bg-secondary hover:border-border/80 hover:-translate-y-0.5" {
                                "Contacto"
                            }
                        }
                    }
                }

                // Características
                section class="max-w-[1200px] mx-auto px-8 py-16" {
                    h2 class="text-2xl font-semibold text-center mb-10" { "¿Por qué NEARS?" }
                    div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6" {
                        (tarjeta_caracteristica(
                            crate::components::icons::regular::lightning("w-8 h-8 text-neaxt-primary"), "Ultrarrápido",
                            "Binario compilado nativo. Sin Virtual DOM, sin hydration, sin overhead."
                        ))
                        (tarjeta_caracteristica(
                            crate::components::icons::regular::lock("w-8 h-8 text-neaxt-primary"), "Seguro por diseño",
                            "Queries SQL verificadas en compilación. Cero CVEs de React/RSC."
                        ))
                        (tarjeta_caracteristica(
                            crate::components::icons::regular::package("w-8 h-8 text-neaxt-primary"), "Binario único",
                            "Toda la app en un archivo. Deploy: copiar y ejecutar."
                        ))
                        (tarjeta_caracteristica(
                            crate::components::icons::regular::folder("w-8 h-8 text-neaxt-primary"), "Estructura familiar",
                            "app/, components/, lib/ — si conoces Next.js, conoces NEARS."
                        ))
                        (tarjeta_caracteristica(
                            crate::components::icons::regular::arrows_clockwise("w-8 h-8 text-neaxt-primary"), "HTMX nativo",
                            "Interactividad sin JavaScript custom. El servidor dicta la verdad."
                        ))
                        (tarjeta_caracteristica(
                            crate::components::icons::regular::check_circle("w-8 h-8 text-neaxt-primary"), "Calidad integrada",
                            "Reglas de arquitectura verificables. Modularidad desde el día 1."
                        ))
                    }
                }
            }

            (footer())
        }
    );

    Html(pagina.into_string())
}

/// Tarjeta de característica.
fn tarjeta_caracteristica(icono: maud::Markup, titulo: &str, descripcion: &str) -> maud::Markup {
    html! {
        div class="bg-card border border-border rounded-xl p-8 flex flex-col gap-3 hover:border-border/80 hover:-translate-y-0.5 hover:shadow-lg" {
            span class="flex items-center justify-center w-12 h-12 rounded-lg bg-neaxt-primary/10 mb-2" {
                (icono)
            }
            h3 class="text-base font-semibold" { (titulo) }
            p class="text-sm text-muted-foreground leading-relaxed" { (descripcion) }
        }
    }
}
