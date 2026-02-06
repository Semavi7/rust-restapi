use std::env;
use dotenvy::dotenv;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_port: String
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

        let db_host = env::var("DB_HOST").expect("DB_HOST set edilmeli");
        let db_user = env::var("DB_USER").expect("DB_USER set edilmeli");
        let db_pass = env::var("DB_PASSWORD").expect("DB_PASSWORD set edilmeli");
        let db_name = env::var("DB_NAME").expect("DB_NAME set edilmeli");
        let db_port = env::var("DB_PORT").expect("DB_PORT set edilmeli");

        let database_url = format!("postgres://{}:{}@{}:{}/{}", db_user, db_pass, db_host, db_port, db_name);

        Config {
            database_url, 
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET set edilmeli"),
            server_port: env::var("SERVER_PORT").unwrap_or_else(|_| ":3000".to_string()),
        }
    }
}