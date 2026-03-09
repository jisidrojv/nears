use maud::{html, Markup};
use crate::lib::types::contacto_types::ContactoEnvio;
use crate::components::ui::menu::{menu, MenuItemProps};

/// Tabla de contactos para dashboard con dropdown de acciones.
pub fn renderizar_tabla(contactos: &[ContactoEnvio]) -> Markup {
    html! {
        @if contactos.is_empty() {
            div class="py-12 text-center" {
                p class="text-muted-foreground text-sm" { "No se encontraron registros." }
            }
        } @else {
            div class="rounded-xl border border-border" {
                table class="w-full text-sm" {
                    thead {
                        tr class="border-b border-border bg-card" {
                            th class="text-left px-4 py-3 font-medium text-muted-foreground" { "#" }
                            th class="text-left px-4 py-3 font-medium text-muted-foreground" { "Nombre" }
                            th class="text-left px-4 py-3 font-medium text-muted-foreground" { "Email" }
                            th class="text-left px-4 py-3 font-medium text-muted-foreground" { "Mensaje" }
                            th class="text-left px-4 py-3 font-medium text-muted-foreground" { "Fecha" }
                            th class="px-4 py-3" { "" }
                        }
                    }
                    tbody {
                        @for contacto in contactos {
                            tr class="border-b border-border last:border-0 hover:bg-accent/50" {
                                td class="px-4 py-3 text-muted-foreground" { (contacto.id) }
                                td class="px-4 py-3 font-medium" {
                                    a href={(format!("/dashboard/{}", contacto.id))}
                                      class="text-foreground hover:text-primary hover:underline" {
                                        (contacto.nombre)
                                    }
                                }
                                td class="px-4 py-3 text-muted-foreground" { (contacto.email) }
                                td class="px-4 py-3 text-muted-foreground max-w-[200px] truncate" { (contacto.mensaje) }
                                td class="px-4 py-3 text-muted-foreground text-xs whitespace-nowrap" {
                                    (contacto.creado.format("%d/%m/%Y %H:%M"))
                                }
                                td class="px-4 py-3 text-right" {
                                    (dropdown_acciones(contacto.id))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Dropdown de acciones por fila usando el componente Menu.
fn dropdown_acciones(id: i32) -> Markup {
    let url_ver = format!("/dashboard/{id}");
    let url_editar = format!("/dashboard/{id}/editar");
    let url_eliminar = format!("/api/contacto/{id}");

    let items = [
        MenuItemProps {
            etiqueta: "Ver detalle",
            icono: Some("👁️"),
            atributos: html! { a href=(url_ver) class="absolute inset-0" {} },
            destructivo: false,
        },
        MenuItemProps {
            etiqueta: "Editar",
            icono: Some("✏️"),
            atributos: html! { a href=(url_editar) class="absolute inset-0" {} },
            destructivo: false,
        },
        MenuItemProps {
            etiqueta: "Eliminar",
            icono: Some("🗑️"),
            atributos: html! {
                button type="button" 
                       x-on:click={"$dispatch('open-dialog', { id: 'dialog-eliminar', url: '" (url_eliminar) "' })"}
                       class="absolute inset-0" {}
            },
            destructivo: true,
        },
    ];

    menu(&format!("menu-{}", id), &items)
}
