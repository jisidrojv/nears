use maud::{html, Markup};

/// Variante del dialog de confirmación.
pub enum ConfirmacionVariante {
    Default,
    Eliminar,
}

/// Dialog de confirmación con botones Cancelar/Confirmar.
pub fn dialog_confirmacion(
    id: &str,
    titulo: &str,
    descripcion: &str,
    variante: ConfirmacionVariante,
) -> Markup {
    let (icono, btn_texto, btn_clase) = match variante {
        ConfirmacionVariante::Default => (
            "✓",
            "Confirmar",
            "bg-gradient-to-r from-primary to-[oklch(0.55_0.2_260)] text-primary-foreground hover:shadow-lg hover:shadow-primary/20",
        ),
        ConfirmacionVariante::Eliminar => (
            "⚠",
            "Eliminar",
            "bg-destructive text-white hover:bg-destructive/90",
        ),
    };

    let evento_apertura = format!(
        "if ($event.detail.id === '{}') {{ url = $event.detail.url || ''; formId = $event.detail.formId || ''; $el.showModal(); }}",
        id
    );

    html! {
        dialog id=(id) 
               x-data="{ url: '', formId: '' }"
               "x-on:open-dialog.window"=(evento_apertura)
               x-on:click="if($event.target === $el) $el.close()"
               class="nears-dialog bg-background border border-border rounded-xl shadow-2xl p-0 backdrop:bg-black/40 backdrop:backdrop-blur-sm max-w-md w-[calc(100%-2rem)] open:animate-fade-in-up" {
            div class="p-6 flex flex-col gap-5" {
                div class="flex flex-col items-center gap-2 text-center" {
                    span class="text-2xl" { (icono) }
                    h2 class="text-lg font-semibold" { (titulo) }
                    p class="text-sm text-muted-foreground" { (descripcion) }
                }
                div class="grid grid-cols-2 gap-3" {
                    button type="button" 
                           x-on:click="$el.closest('dialog').close()"
                           class="inline-flex items-center justify-center px-4 py-2.5 rounded-lg text-sm font-medium bg-accent text-foreground border border-border hover:bg-secondary" {
                        "Cancelar"
                    }
                    button type="button"
                           x-on:click=(match variante {
                               ConfirmacionVariante::Eliminar => "$el.closest('dialog').close(); htmx.ajax('DELETE', url, { target: '#dashboard-contenido', swap: 'innerHTML' })",
                               _ => "$el.closest('dialog').close(); if(formId) { htmx.trigger(document.getElementById(formId), 'submit') }"
                           })
                           class={"inline-flex items-center justify-center px-4 py-2.5 rounded-lg text-sm font-semibold " (btn_clase)} {
                        (btn_texto)
                    }
                }
            }
        }
    }
}
