use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm::schema::Schema;
use sea_orm::{ConnectionTrait, DbBackend, Statement};
use crate::models::{user, todo};

pub async fn connect(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(db_url).await?;
    println!("Veritabanı bağlantısı başarılı!");
    create_table_if_not_exists(&db).await?;
    Ok(db)
}

async fn create_table_if_not_exists(db: &DatabaseConnection) -> Result<(), DbErr>{
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let stmt_user = builder.build(&schema.create_table_from_entity(user::Entity));
    match  db.execute(stmt_user).await {
        Ok(_) => println!("User tablosu kontrol edildi/oluşturuldu."),
        Err(e) => println!("Tablo hatası (User): {}", e),
    }

    let stmt_todo = builder.build(&schema.create_table_from_entity(todo::Entity));
    match db.execute(stmt_todo).await {
        Ok(_) => println!("Todo tablosu kontrol edildi/oluşturuldu."),
        Err(e) => println!("Tablo hatası (Todo): {}", e),
    }

    Ok(())
}