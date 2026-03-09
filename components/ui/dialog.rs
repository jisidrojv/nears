use maud::{html, Markup};

/// Dialog genérico usando <dialog> nativo del navegador.
pub fn dialog(id: &str, titulo: &str, descripcion: Option<&str>, contenido: Markup) -> Markup {
    html! {
        dialog id=(id) class="nears-dialog bg-background border border-border rounded-xl shadow-2xl p-0 backdrop:bg-black/40 backdrop:backdrop-blur-sm max-w-md w-[calc(100%-2rem)] open:animate-fade-in-up" {
            div class="p-6 flex flex-col gap-4" {
                // Header
                div class="flex flex-col gap-1 text-center" {
                    h2 class="text-lg font-semibold" { (titulo) }
                    @if let Some(desc) = descripcion {
                        p class="text-sm text-muted-foreground" { (desc) }
                    }
                }
                // Content
                (contenido)
            }
        }
    }
}

/// Dialog con formulario de edición (contenido dinámico via HTMX).
pub fn dialog_contenedor(id: &str) -> Markup {
    html! {
        dialog id=(id) class="nears-dialog bg-background border border-border rounded-xl shadow-2xl p-0 backdrop:bg-black/40 backdrop:backdrop-blur-sm max-w-md w-[calc(100%-2rem)] open:animate-fade-in-up" {
            div id={(id) "-contenido"} class="p-6" {
                // Se llenará via HTMX
            }
        }
    }
}
