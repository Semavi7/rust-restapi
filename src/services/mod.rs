use std::sync::Arc;
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::models::{todo, user};
use crate::repositories::{TodoRepository, UserRepository};
use crate::config::Config;

#[derive(Deserialize)]
pub struct TodoCreateDTO {
    pub title: String,
}

#[derive(Deserialize)]
pub struct AuthDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: i32,
    pub role: String,
    pub exp: usize,
}

#[async_trait]
pub trait TodoService: Send + Sync {
    async fn get_all(&self) -> Result<Vec<todo::Model>, String>;
    async fn create(&self, dto: TodoCreateDTO) -> Result<todo::Model, String>;
    async fn get_by_id(&self, id: i32) -> Result<todo::Model, String>;
}

pub struct TodoServiceImpl {
    repo: Arc<dyn TodoRepository>,
}

impl TodoServiceImpl {
    pub fn new(repo: Arc<dyn TodoRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    async fn get_all(&self) -> Result<Vec<todo::Model>, String> {
        self.repo.get_all_todos().await
    }

    async fn create(&self, dto: TodoCreateDTO) -> Result<todo::Model, String> {
        // Validasyon
        if dto.title.len() < 3 {
            return Err("Başlık en az 3 karakter olmalı".to_string());
        }
        self.repo.create_todo(dto.title).await
    }

    async fn get_by_id(&self, id: i32) -> Result<todo::Model, String> {
        match self.repo.get_todo_by_id(id).await? {
            Some(todo) => Ok(todo),
            None => Err("Todo bulunamadı".to_string()),
        }
    }
}


#[async_trait]
pub trait AuthService: Send + Sync {
    async fn register(&self, dto: AuthDTO) -> Result<user::Model, String>;
    async fn login(&self, dto: AuthDTO) -> Result<String, String>;
}

pub struct AuthServiceImpl {
    repo: Arc<dyn UserRepository>,
    config: Config,
}

impl AuthServiceImpl {
    pub fn new(repo: Arc<dyn UserRepository>, config: Config) -> Self {
        Self { repo, config }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn register(&self, dto: AuthDTO) -> Result<user::Model, String> {
        let hashed_pass = hash(dto.password, DEFAULT_COST).map_err(|_| "Şifre hashlenemedi")?;
        self.repo.create_user(dto.email, hashed_pass).await
    }

    async fn login(&self, dto: AuthDTO) -> Result<String, String> {
        let user = self.repo.find_by_email(dto.email.clone()).await?
            .ok_or("Kullanıcı bulunamadı")?;

        if !verify(dto.password, &user.password).unwrap_or(false) {
            return Err("Hatalı şifre".to_string());
        }

        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(72))
            .expect("Tarih hatası")
            .timestamp() as usize;

        let claims = Claims {
            user_id: user.id,
            role: user.role,
            exp: expiration,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()))
            .map_err(|_| "Token oluşturulamadı".to_string())
    }
}