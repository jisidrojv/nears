use maud::{html, Markup};

/// Tarjeta de estadística.
pub fn card_stat(icono: &str, titulo: &str, valor: &str, detalle: Option<&str>) -> Markup {
    html! {
        div class="relative bg-card border border-border rounded-xl p-5 flex flex-col gap-2 overflow-hidden" {
            // Icono decorativo fondo
            div class="absolute top-2 right-3 text-3xl opacity-10 pointer-events-none" {
                (icono)
            }
            // Título
            p class="text-sm font-medium text-muted-foreground" { (titulo) }
            // Valor
            p class="text-2xl font-semibold tabular-nums" { (valor) }
            // Detalle
            @if let Some(det) = detalle {
                p class="text-xs text-muted-foreground" { (det) }
            }
        }
    }
}
