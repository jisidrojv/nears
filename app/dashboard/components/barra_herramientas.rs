use maud::{html, Markup};

/// Barra de herramientas: búsqueda + select de ordenación.
pub fn barra_herramientas(busqueda: &str, orden: &str) -> Markup {
    html! {
        div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-3" {
            // Input de búsqueda con HTMX
            div class="flex items-center w-full sm:w-[280px] h-9 rounded-lg border border-border bg-card overflow-hidden focus-within:ring-2 focus-within:ring-ring" {
                div class="flex h-9 items-center justify-center pl-3 shrink-0" {
                    svg class="w-4 h-4 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" {
                        path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" {}
                    }
                }
                input type="text" name="busqueda" value=(busqueda)
                      placeholder="Buscar..."
                      hx-get="/api/dashboard"
                      hx-target="#dashboard-contenido"
                      hx-swap="innerHTML"
                      hx-trigger="input changed delay:300ms, search"
                      hx-include="[name='orden']"
                      class="flex-1 min-w-0 h-9 bg-transparent px-3 text-sm outline-none placeholder:text-muted-foreground";
            }

            // Contenedor derecho: Select y Crear
            div class="flex items-center gap-3 w-full sm:w-auto mt-3 sm:mt-0" {
                // Select de ordenación
                div class="relative flex-1 sm:flex-none" {
                    select name="orden"
                           hx-get="/api/dashboard"
                           hx-target="#dashboard-contenido"
                           hx-swap="innerHTML"
                           hx-trigger="change"
                           hx-include="[name='busqueda']"
                           class="w-full sm:w-auto appearance-none h-9 pl-3 pr-8 rounded-lg border border-border bg-card text-sm text-foreground cursor-pointer hover:bg-accent/50 focus:outline-none focus:ring-2 focus:ring-ring" {
                        option value="reciente" selected[orden == "reciente"] { "Más recientes" }
                        option value="antiguo" selected[orden == "antiguo"] { "Más antiguos" }
                        option value="nombre" selected[orden == "nombre"] { "Nombre A-Z" }
                        option value="email" selected[orden == "email"] { "Email A-Z" }
                    }
                    // Flecha del select
                    div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2" {
                        svg class="w-4 h-4 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" {
                            path stroke-linecap="round" stroke-linejoin="round" d="M8 9l4-4 4 4m0 6l-4 4-4-4" {}
                        }
                    }
                }
                
                // Botón Nuevo
                a href="/dashboard/crear"
                   class="inline-flex h-9 items-center justify-center gap-2 px-4 rounded-lg text-sm font-medium bg-primary text-primary-foreground hover:bg-primary/90 shrink-0 shadow-sm" {
                    span { "✨ Nuevo" }
                }
            }
        }
    }
}
