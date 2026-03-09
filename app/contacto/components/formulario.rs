use maud::{html, Markup};

/// Sección de formulario de contacto con confirmación.
pub fn formulario_contacto() -> Markup {
    html! {
        section {
            h1 class="text-3xl font-bold mb-2" { "Contacto" }
            p class="text-muted-foreground mb-6" { "Envía un mensaje y se guardará en PostgreSQL." }

            form id="form-contacto"
                 hx-post="/api/contacto"
                 hx-swap="none"
                 hx-on--after-request="if(event.detail.successful) this.reset()" {

                div class="flex flex-col gap-4" {
                    (campo_texto("nombre", "Nombre", "text", "Tu nombre"))
                    (campo_texto("email", "Email", "email", "tu@email.com"))

                    div class="flex flex-col gap-1.5" {
                        label for="mensaje" class="text-sm font-medium" { "Mensaje" }
                        textarea name="mensaje" id="mensaje" required rows="4"
                                 placeholder="Escribe tu mensaje..."
                                 class="w-full px-4 py-2.5 bg-card border border-border rounded-lg text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring resize-none" {}
                    }

                    button type="button"
                           x-data=""
                           x-on:click="$dispatch('open-dialog', { id: 'dialog-confirmar', formId: 'form-contacto' })"
                           class="w-full inline-flex items-center justify-center gap-2 py-2.5 rounded-lg text-sm font-semibold bg-gradient-to-r from-primary to-[oklch(0.55_0.2_260)] text-primary-foreground disabled:opacity-50 disabled:pointer-events-none"
                           data-loading="" {
                        span class="nears-spinner" { (spinner_svg()) }
                        span class="nears-btn-text" { "Enviar mensaje" }
                    }
                }
            }
        }
    }
}

/// Campo de texto reutilizable dentro del formulario.
fn campo_texto(nombre: &str, etiqueta: &str, tipo: &str, placeholder: &str) -> Markup {
    html! {
        div class="flex flex-col gap-1.5" {
            label for=(nombre) class="text-sm font-medium" { (etiqueta) }
            input type=(tipo) name=(nombre) id=(nombre) required
                  placeholder=(placeholder)
                  class="w-full px-4 py-2.5 bg-card border border-border rounded-lg text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring";
        }
    }
}

/// SVG de spinner para estados de carga.
pub fn spinner_svg() -> Markup {
    html! {
        svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" {
            circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" {}
            path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" {}
        }
    }
}
