use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json, http::StatusCode,
};
use axum_extra::extract::cookie::{Cookie, SameSite, CookieJar};
use std::sync::Arc;
use crate::AppState;
use crate::services::{TodoCreateDTO, AuthDTO};

// --- AUTH HANDLERS ---
pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<AuthDTO>,
) -> impl IntoResponse {
    match state.auth_service.register(dto).await {
        Ok(user) => (StatusCode::CREATED, Json(serde_json::json!(user))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(dto): Json<AuthDTO>,
) -> impl IntoResponse {
    match state.auth_service.login(dto).await {
        Ok(token) => {
            let cookie = Cookie::build(("jwt", token))
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .build();
            
            let updated_jar = jar.add(cookie);
            (StatusCode::OK, updated_jar, Json(serde_json::json!({"message": "Giriş başarılı"}))).into_response()
        },
        Err(e) => (StatusCode::UNAUTHORIZED, jar, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// --- TODO HANDLERS ---
pub async fn get_todos_handler(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match state.todo_service.get_all().await {
        Ok(todos) => Json(todos).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

pub async fn create_todo_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<TodoCreateDTO>,
) -> impl IntoResponse {
    match state.todo_service.create(dto).await {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

pub async fn get_todo_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match state.todo_service.get_by_id(id).await {
        Ok(todo) => Json(todo).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": e}))).into_response(),
    }
}