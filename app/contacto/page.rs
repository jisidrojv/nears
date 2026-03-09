use axum::extract::Extension;
use axum::response::Html;
use maud::html;
use sqlx::PgPool;
use crate::app::layout::layout_raiz;
use crate::components::header::{header, HeaderProps};
use crate::components::footer::footer;
use crate::components::ui::dialog_confirmation::{dialog_confirmacion, ConfirmacionVariante};
use crate::app::contacto::components::formulario::formulario_contacto;

/// Página de contacto — Ruta: /contacto
/// Solo formulario de envío. La tabla está en /dashboard.
pub async fn render(Extension(_pool): Extension<PgPool>) -> Html<String> {
    let props_header = HeaderProps {
        titulo: "NEARS",
        mostrar_nav: true,
        session: None,
    };

    let pagina = layout_raiz("Contacto | NEARS",
        html! {
            (header(&props_header))
            main class="max-w-[600px] mx-auto px-8 py-12" {
                (formulario_contacto())
                div id="tabla-contactos" class="mt-6" {}
            }
            (footer())
            (dialog_confirmacion("dialog-confirmar", "Confirmar envío", "Se registrarán los datos. ¿Deseas continuar?", ConfirmacionVariante::Default))
        }
    );

    Html(pagina.into_string())
}
