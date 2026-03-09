use maud::{html, Markup, DOCTYPE};
use crate::components::ui::toast::toaster;

/// Layout raíz — envuelve TODAS las páginas de la aplicación.
pub fn layout_raiz(titulo: &str, contenido: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (titulo) }
                link rel="stylesheet" href="/static/style.css";
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="";
                link rel="stylesheet"
                     href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap";
                style {
                    (maud::PreEscaped("[x-cloak] { display: none !important; }"))
                }
                script {
                    (maud::PreEscaped(r#"
                        try {
                            if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
                                document.documentElement.classList.add('dark');
                            } else {
                                document.documentElement.classList.remove('dark');
                            }
                        } catch (_) {}
                    "#))
                }
                script src="/static/js/htmx.min.js" {}
                script defer src="/static/js/alpine.min.js" {}
            }
            body class="min-h-screen" {
                (contenido)
                (toaster())
            }
        }
    }
}
