use maud::{html, Markup};

/// Item individual del menú.
pub struct MenuItemProps<'a> {
    pub etiqueta: &'a str,
    pub icono: Option<&'a str>,
    pub atributos: Markup,
    pub destructivo: bool,
}

/// Menú dropdown para acciones.
pub fn menu(_id: &str, items: &[MenuItemProps]) -> Markup {
    let clase_base = "flex items-center gap-2 w-full px-3 py-2 text-sm rounded-md cursor-pointer relative";

    html! {
        div x-data="{ open: false }" class="relative" {
            button type="button"
                   x-on:click="open = !open"
                   class="inline-flex items-center justify-center w-8 h-8 rounded-md text-muted-foreground hover:bg-accent hover:text-foreground" {
                // Tres puntos verticales
                svg class="w-4 h-4" fill="currentColor" viewBox="0 0 16 16" {
                    circle cx="8" cy="3" r="1.5" {}
                    circle cx="8" cy="8" r="1.5" {}
                    circle cx="8" cy="13" r="1.5" {}
                }
            }
            div x-show="open"
                "x-on:click.outside"="open = false"
                x-cloak=""
                class="absolute right-0 top-full mt-1 z-50 min-w-[160px] bg-card border border-border rounded-lg shadow-lg p-1.5" {
                @for item in items {
                    @if item.destructivo {
                        div class={(clase_base) " text-destructive hover:bg-destructive/10"} 
                            x-on:click="open = false" {
                            (item.atributos)
                            @if let Some(icono) = item.icono {
                                span { (icono) }
                            }
                            span { (item.etiqueta) }
                        }
                    } @else {
                        div class={(clase_base) " text-foreground hover:bg-accent"} 
                            x-on:click="open = false" {
                            (item.atributos)
                            @if let Some(icono) = item.icono {
                                span { (icono) }
                            }
                            span { (item.etiqueta) }
                        }
                    }
                }
            }
        }
    }
}
