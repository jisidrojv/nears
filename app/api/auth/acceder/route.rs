use axum::{
    extract::{Extension, Form},
    response::Redirect,
};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use sqlx::PgPool;
use crate::lib::auth::{compare_password, create_session_cookie, no_store_response};

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub async fn post(
    Extension(pool): Extension<PgPool>,
    jar: CookieJar,
    Form(payload): Form<LoginForm>,
) -> axum::response::Response {
    // 1. Buscar al usuario por correo
    let user_query = sqlx::query!(
        r#"
        SELECT u.id, u.password_hash, u.email, r.nombre as rol_nombre 
        FROM usuarios u
        JOIN roles r ON u.rol_id = r.id
        WHERE u.email = $1 AND u.activo = true
        "#,
        payload.email
    )
    .fetch_optional(&pool)
    .await;

    match user_query {
        Ok(Some(user)) => {
            // 2. Verificar la contraseña usando bcrypt
            if compare_password(&payload.password, &user.password_hash) {
                // 3. Contraseña válida -> Crear cookie de sesión DAL
                let cookie = create_session_cookie(user.id, user.email, user.rol_nombre);
                
                // Retornar redirección inyectando la cookie al navegador
                return no_store_response((jar.add(cookie), Redirect::to("/dashboard")));
            }
        }
        Ok(None) | Err(_) => {
            // Usuario no encontrado o inactivo
        }
    }

    // 4. Si algo falla, redirigir a /acceder con un posible mensaje temporal (a futuro)
    no_store_response(Redirect::to("/acceder"))
}
