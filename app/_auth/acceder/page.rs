use axum_extra::extract::cookie::CookieJar;
use axum::response::{Html, Redirect};
use maud::html;
use crate::app::layout::layout_raiz;
use crate::components::icons::regular;
use crate::lib::auth::{no_store_response, verify_session};

pub async fn render(jar: CookieJar) -> axum::response::Response {
    // Si ya existe sesión, mandarlo al dashboard sin preguntar
    if verify_session(&jar).is_some() {
        return no_store_response(Redirect::to("/dashboard"));
    }

    let pagina = layout_raiz("Acceder - NEARS",
        html! {
            main class="flex items-center justify-center min-h-[calc(100vh-100px)] px-4" {
                div class="w-full max-w-md bg-card border border-border rounded-xl p-8 shadow-lg" {
                    
                    div class="flex flex-col items-center mb-8" {
                        span class="flex items-center justify-center w-16 h-16 rounded-full bg-neaxt-primary/10 mb-4" {
                            (regular::user("w-8 h-8 text-neaxt-primary"))
                        }
                        h1 class="text-2xl font-bold tracking-tight text-foreground" {
                            "Iniciar Sesión"
                        }
                        p class="text-sm text-muted-foreground mt-2" {
                            "Ingresa a tu cuenta para continuar"
                        }
                    }

                    form action="/api/auth/acceder" method="POST" class="space-y-4" {
                        div class="space-y-2" {
                            label class="text-sm font-medium leading-none text-foreground" for="email" {
                                "Correo electrónico"
                            }
                            input type="email" id="email" name="email" required
                                class="flex h-10 w-full rounded-md border border-border bg-input px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-neaxt-primary focus:border-transparent" 
                                placeholder="tu@email.com" {}
                        }
                        
                        div class="space-y-2" {
                            div class="flex items-center justify-between" {
                                label class="text-sm font-medium leading-none text-foreground" for="password" {
                                    "Contraseña"
                                }
                                a href="#" class="text-xs text-neaxt-primary hover:underline" {
                                    "¿Olvidaste tu contraseña?"
                                }
                            }
                            input type="password" id="password" name="password" required
                                class="flex h-10 w-full rounded-md border border-border bg-input px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-neaxt-primary focus:border-transparent" 
                                placeholder="••••••••" {}
                        }

                        button type="submit" 
                            class="w-full inline-flex items-center justify-center h-10 px-4 py-2 mt-4 rounded-md text-sm font-semibold bg-neaxt-primary text-primary-foreground hover:bg-neaxt-primary/90 transition-colors" {
                            "Acceder"
                        }
                    }

                    div class="mt-6 text-center text-sm text-muted-foreground" {
                        "¿No tienes una cuenta? "
                        a href="#" class="text-neaxt-primary font-medium hover:underline" {
                            "Regístrate"
                        }
                    }
                }
            }
        }
    );

    no_store_response(Html(pagina.into_string()))
}
