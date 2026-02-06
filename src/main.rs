mod config;
mod database;
mod models;
mod repositories;
mod services;
mod middleware;
mod handlers;

use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use crate::{config::Config, middleware::auth::auth_middleware, middleware::auth::admin_only};
use crate::repositories::{TodoRepositoryImpl, UserRepositoryImpl};
use crate::services::{TodoService, TodoServiceImpl, AuthService, AuthServiceImpl};

pub struct AppState {
    pub config: Config,
    pub todo_service: Arc<dyn TodoService>,
    pub auth_service: Arc<dyn AuthService>,
}

#[tokio::main]
async fn main() {
    // 1. Config Yükle
    let config = Config::init();

    // 2. Veritabanı Bağlantısı
    let db = database::connect(&config.database_url).await.expect("Veritabanı hatası");

    // 3. DI (Dependency Injection) Zinciri
    let todo_repo = Arc::new(TodoRepositoryImpl::new(db.clone()));
    let todo_service = Arc::new(TodoServiceImpl::new(todo_repo));

    let user_repo = Arc::new(UserRepositoryImpl::new(db.clone()));
    let auth_service = Arc::new(AuthServiceImpl::new(user_repo, config.clone()));

    // 4. App State Oluştur
    let app_state = Arc::new(AppState {
        config: config.clone(),
        todo_service,
        auth_service,
    });

    // 5. Rotaları Tanımla
    // Public Rotalar
    let auth_routes = Router::new()
        .route("/register", post(handlers::register_handler))
        .route("/login", post(handlers::login_handler));

    // Korumalı Rotalar (Middleware ile)
    let protected_routes = Router::new()
        .route("/todos", get(handlers::get_todos_handler))
        .route("/todos/:id", get(handlers::get_todo_handler))
        // Sadece Admin (Create Todo)
        .route("/admin/todos", post(handlers::create_todo_handler).layer(axum::middleware::from_fn(admin_only)))
        .layer(axum::middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    let app = Router::new()
        .nest("/api/auth", auth_routes)
        .nest("/api", protected_routes)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(app_state);

    // 6. Sunucuyu Başlat
    let listener = tokio::net::TcpListener::bind(&config.server_port).await.unwrap();
    println!("Sunucu {} portunda çalışıyor...", config.server_port);
    axum::serve(listener, app).await.unwrap();
}
