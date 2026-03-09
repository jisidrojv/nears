use crate::lib::types::contacto_types::ContactoEnvio;
use maud::{html, Markup};

/// Formulario compartido para crear y editar.
pub fn formulario_contacto(contacto: Option<&ContactoEnvio>) -> Markup {
    let es_edicion = contacto.is_some();

    // Determinar la URL de la acción.
    let url_accion = if let Some(c) = contacto {
        format!("/api/contacto/{}", c.id) // PUT
    } else {
        "/api/contacto".to_string() // POST
    };

    html! {
        div class="flex flex-col gap-6" {
            // alpine state para isDirty
            @let campos = html! {
                div class="grid grid-cols-1 md:grid-cols-2 gap-6" {
                    div class="flex flex-col gap-4 col-span-1 md:col-span-2" {
                        div class="flex flex-col gap-1.5" {
                            label class="text-sm font-medium" { "Nombre" }
                            input type="text" name="nombre" required
                                  value=(contacto.map(|c| c.nombre.as_str()).unwrap_or(""))
                                  class="w-full px-4 py-2.5 bg-card border border-border rounded-lg text-foreground focus:outline-none focus:ring-2 focus:ring-ring";
                        }
                        div class="flex flex-col gap-1.5" {
                            label class="text-sm font-medium" { "Email" }
                            input type="email" name="email" required
                                  value=(contacto.map(|c| c.email.as_str()).unwrap_or(""))
                                  class="w-full px-4 py-2.5 bg-card border border-border rounded-lg text-foreground focus:outline-none focus:ring-2 focus:ring-ring";
                        }
                        div class="flex flex-col gap-1.5" {
                            label class="text-sm font-medium" { "Mensaje" }
                            textarea name="mensaje" required rows="4"
                                     class="w-full px-4 py-2.5 bg-card border border-border rounded-lg text-foreground focus:outline-none focus:ring-2 focus:ring-ring resize-none" {
                                (contacto.map(|c| c.mensaje.as_str()).unwrap_or(""))
                            }
                        }
                    }

                    div class="col-span-1 md:col-span-2 flex items-center justify-end gap-3 pt-4 border-t border-border" {
                        a href="/dashboard"
                           class="px-4 py-2.5 rounded-lg text-sm font-medium bg-accent text-foreground border border-border hover:bg-secondary" {
                            "Cancelar"
                        }
                        button type="submit" data-loading=""
                               x-bind:disabled={@if es_edicion { "!isDirty" } @else { "false" } }
                               class="inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg text-sm font-semibold bg-gradient-to-r from-primary to-[oklch(0.55_0.2_260)] text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed" {
                            span class="nears-btn-text" {
                                @if es_edicion { "Guardar cambios" } @else { "Crear contacto" }
                            }
                        }
                    }
                }
            };

            @if es_edicion {
                form hx-put=(url_accion)
                     hx-swap="none"
                     x-data="{ original: '', isDirty: false, initUrl() { this.original = new URLSearchParams(new FormData(this.$el)).toString(); }, checkDirty() { this.isDirty = new URLSearchParams(new FormData(this.$el)).toString() !== this.original; } }"
                     x-init="initUrl()"
                     x-on:input="checkDirty()"
                     "x-on:htmx:after-request"="if($event.detail.successful) { initUrl(); checkDirty(); }" {
                    (campos)
                }
            } @else {
                form hx-post="/api/contacto?source=dashboard"
                     hx-swap="none"
                     x-data="{ isDirty: false, checkDirty() { this.isDirty = true; } }"
                     x-on:input="checkDirty()" {
                    (campos)
                }
            }
        }
    }
}
