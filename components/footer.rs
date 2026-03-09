use maud::{html, Markup};

/// Componente Footer reutilizable.
pub fn footer() -> Markup {
    html! {
        footer class="border-t border-border mt-16" {
            div class="max-w-[1200px] mx-auto px-8 py-8 flex flex-col items-center gap-2 text-center" {
                p class="text-sm text-muted-foreground" {
                    "Construido con "
                    span class="text-primary font-semibold" { "NEARS" }
                    " — Rust nativo, cero dependencias en el cliente."
                }
                p class="text-xs text-muted-foreground/60" {
                    "© 2026 NEARS Framework"
                }
            }
        }
    }
}
