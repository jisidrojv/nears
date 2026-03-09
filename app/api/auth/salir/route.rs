use axum_extra::extract::cookie::CookieJar;
use axum::response::Redirect;
use crate::lib::auth::{delete_session_cookie, no_store_response};

pub async fn get(jar: CookieJar) -> axum::response::Response {
    let empty_cookie = delete_session_cookie();
    
    // Inyecta la cookie de sesión en blanco/expirada para destruir la sesión real en el cliente
    no_store_response((jar.add(empty_cookie), Redirect::to("/acceder")))
}
