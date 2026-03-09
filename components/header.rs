use maud::{html, Markup};
use crate::components::ui::theme_toggle::theme_toggle;
use crate::components::icons::regular;
use crate::lib::auth::SessionPayload;

pub struct HeaderProps<'a> {
    pub titulo: &'a str,
    pub mostrar_nav: bool,
    pub session: Option<SessionPayload>, // Opcional, inyectado si existe
}

/// Componente Header reutilizable.
pub fn header(props: &HeaderProps) -> Markup {
    html! {
        header class="sticky top-0 z-50 bg-background/85 backdrop-blur-2xl border-b border-border" {
            div class="max-w-[1200px] mx-auto px-8 py-4 flex items-center justify-between" {
                a href="/" class="flex items-center gap-2 font-bold text-lg text-foreground hover:opacity-80 transition-opacity" {
                    span class="text-2xl" { "🦀" }
                    span { (props.titulo) }
                }
                @if props.mostrar_nav {
                    nav class="flex items-center gap-2 relative" {
                        a href="/" class="px-3 py-2 rounded-md text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-accent" { "Inicio" }
                        a href="/dashboard" class="px-3 py-2 rounded-md text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-accent" { "Dashboard" }
                        a href="/contacto" class="px-3 py-2 rounded-md text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-accent" { "Contacto" }
                        
                        div class="w-px h-5 bg-border mx-1" {} // Separador vertical

                        (theme_toggle())
                        
                        // Validar si tiene sesión para mostrar User/Login o Logout
                        @if let Some(ref ses) = props.session {
                            div class="relative ml-2 group" {
                                a href="/api/auth/salir" class="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium text-red-500 hover:bg-red-500/10 transition-colors" title=(format!("Cerrar sesión ({})", ses.email)) {
                                    (regular::sign_out("w-5 h-5"))
                                    span class="hidden sm:inline" { "Salir" }
                                }
                            }
                        } @else {
                            a href="/acceder" class="flex items-center justify-center p-2 rounded-full text-muted-foreground hover:text-foreground hover:bg-accent transition-colors ml-2" title="Acceder" {
                                (regular::user_circle("w-5 h-5"))
                                span class="sr-only" { "Acceder" }
                            }
                        }
                    }
                }
            }
        }
    }
}
