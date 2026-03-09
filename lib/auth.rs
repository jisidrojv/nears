use axum::{
    http::{header, HeaderMap, HeaderValue},
    response::{Html, IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

/// Payload que guardaremos dentro del JWT de la sesión
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionPayload {
    pub user_id: i32,
    pub email: String,
    pub rol: String,
    pub exp: usize,
}

/// Extraer el JWT_SECRET, idealmente del .env
fn get_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key-change-in-production".to_string())
}

/// Crea una sesión JWT y retorna la cookie configurada lista para inyectarse
pub fn create_session_cookie(user_id: i32, email: String, rol: String) -> Cookie<'static> {
    // 7 días de duración
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let payload = SessionPayload {
        user_id,
        email,
        rol,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(get_secret().as_ref()),
    )
    .expect("Fallo al generar JWT");

    let mut cookie = Cookie::new("session", token);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_same_site(axum_extra::extract::cookie::SameSite::Lax);
    cookie.set_max_age(cookie::time::Duration::days(7));

    cookie
}

/// Verifica la sesión leyendo la CookieJar de la request (Patrón DAL Auth equivalente a Next.js)
pub fn verify_session(jar: &CookieJar) -> Option<SessionPayload> {
    let cookie = jar.get("session")?;
    let token = cookie.value();

    let secret = get_secret();
    let validation = Validation::default();

    match decode::<SessionPayload>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(token_data) => Some(token_data.claims),
        Err(_) => None,
    }
}

/// Fuerza que la respuesta no sea cacheada para evitar HTML obsoleto en rutas sensibles.
pub fn no_store_response<T: IntoResponse>(value: T) -> Response {
    let mut response = value.into_response();
    let headers = response.headers_mut();

    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, no-cache, must-revalidate, private"),
    );
    headers.insert(header::PRAGMA, HeaderValue::from_static("no-cache"));
    headers.insert(header::EXPIRES, HeaderValue::from_static("0"));

    response
}

/// Guard estándar para páginas privadas.
pub fn require_session(jar: &CookieJar) -> Result<SessionPayload, Response> {
    verify_session(jar).ok_or_else(|| no_store_response(Redirect::to("/acceder")))
}

/// Guard estándar para endpoints HTMX privados.
pub fn require_htmx_session(jar: &CookieJar) -> Result<SessionPayload, Response> {
    verify_session(jar).ok_or_else(|| htmx_redirect_response("/acceder"))
}

/// Redirección compatible con HTMX para evitar swaps inconsistentes.
pub fn htmx_redirect_response(path: &str) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert("HX-Redirect", HeaderValue::from_str(path).expect("Ruta HTMX válida"));

    no_store_response((headers, Html(String::new())))
}

/// Crea una cookie efímera segura para mostrar un toast después de una redirección completa.
pub fn create_toast_cookie(message: &str, kind: &str) -> Cookie<'static> {
    let encoded = percent_encode_cookie_value(&format!("{message}|{kind}"));

    let mut cookie = Cookie::new("nears_toast", encoded);
    cookie.set_path("/");
    cookie.set_max_age(cookie::time::Duration::seconds(10));

    cookie
}

fn percent_encode_cookie_value(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());

    for byte in value.bytes() {
        let is_safe = matches!(
            byte,
            b'A'..=b'Z'
                | b'a'..=b'z'
                | b'0'..=b'9'
                | b'-'
                | b'_'
                | b'.'
                | b'~'
                | b'|'
        );

        if is_safe {
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            encoded.push_str(&format!("{byte:02X}"));
        }
    }

    encoded
}

/// Retorna una cookie vacía y expirada para destruir la sesión
pub fn delete_session_cookie() -> Cookie<'static> {
    let mut cookie = Cookie::new("session", "");
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_max_age(cookie::time::Duration::seconds(0));
    cookie
}

/// Hashea una contraseña usando bcrypt
#[allow(dead_code)]
pub fn hash_password(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).expect("No se pudo hashear la contraseña")
}

/// Compara una contraseña en texto plano contra su hash
pub fn compare_password(password: &str, hashed: &str) -> bool {
    bcrypt::verify(password, hashed).unwrap_or(false)
}
