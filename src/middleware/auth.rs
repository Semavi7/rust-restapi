use axum::{
    body::Body,
    extract::Request,
    http::{StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::services::Claims;
use crate::config::Config;
use std::sync::Arc;

// 1. JWT Koruması (Protected)
pub async fn auth_middleware(
    state: axum::extract::State<Arc<crate::AppState>>, // Config'e erişmek için
    jar: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = jar.get("jwt").map(|cookie| cookie.value().to_string());

    if let Some(token) = token {
        let secret = &state.config.jwt_secret;
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        );

        match token_data {
            Ok(data) => {
                // Claims'i request context'e ekle (Admin kontrolü için)
                req.extensions_mut().insert(data.claims);
                Ok(next.run(req).await)
            }
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

// 2. Admin Koruması (AdminOnly)
pub async fn admin_only(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = req.extensions().get::<Claims>();

    if let Some(claims) = claims {
        if claims.role == "admin" {
            Ok(next.run(req).await)
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}