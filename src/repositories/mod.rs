use crate::models::{todo, user};
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get_all_todos(&self) -> Result<Vec<todo::Model>, String>;
    async fn create_todo(&self, title: String) -> Result<todo::Model, String>;
    async fn get_todo_by_id(&self, id: i32) -> Result<Option<todo::Model>, String>;
}

pub struct TodoRepositoryImpl {
    db: DatabaseConnection,
}

impl TodoRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn get_all_todos(&self) -> Result<Vec<todo::Model>, String> {
        todo::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())
    }

    async fn create_todo(&self, title: String) -> Result<todo::Model, String> {
        let new_todo = todo::ActiveModel {
            title: Set(title),
            completed: Set(false),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        new_todo.insert(&self.db).await.map_err(|e| e.to_string())
    }

    async fn get_todo_by_id(&self, id: i32) -> Result<Option<todo::Model>, String> {
        todo::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, email: String, password: String) -> Result<user::Model, String>;
    async fn find_by_email(&self, email: String) -> Result<Option<user::Model>, String>;
}

pub struct UserRepositoryImpl {
    db: DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, email: String, password: String) -> Result<user::Model, String> {
        let new_user = user::ActiveModel {
            email: Set(email),
            password: Set(password),
            role: Set("user".to_string()),
            ..Default::default()
        };
        new_user.insert(&self.db).await.map_err(|e| e.to_string())
    }

    async fn find_by_email(&self, email: String) -> Result<Option<user::Model>, String> {
        use sea_orm::ColumnTrait;
        use sea_orm::QueryFilter;

        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())
    }
}
