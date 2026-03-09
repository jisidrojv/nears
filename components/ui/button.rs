use maud::{html, Markup};

/// Botón con soporte de loading spinner via data-loading + HTMX hooks.
pub fn boton(texto: &str, variante: &str, tipo: &str) -> Markup {
    let clase_variante = match variante {
        "primary" => "bg-gradient-to-r from-primary to-[oklch(0.55_0.2_260)] text-primary-foreground shadow-lg shadow-primary/20 hover:-translate-y-0.5 hover:shadow-xl hover:shadow-primary/30",
        "destructive" => "bg-destructive text-white hover:bg-destructive/90",
        "outline" => "bg-accent text-foreground border border-border hover:bg-secondary hover:border-border/80",
        "ghost" => "hover:bg-accent hover:text-accent-foreground",
        _ => "bg-primary text-primary-foreground hover:bg-primary/90",
    };

    html! {
        button type=(tipo) data-loading=""
               class={"inline-flex items-center justify-center gap-2 px-6 py-2.5 rounded-lg text-sm font-semibold disabled:opacity-50 disabled:pointer-events-none " (clase_variante)} {
            // Spinner (visible solo durante loading via CSS)
            span class="nears-spinner" {
                svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" {
                    circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" {}
                    path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" {}
                }
            }
            span class="nears-btn-text" { (texto) }
        }
    }
}
