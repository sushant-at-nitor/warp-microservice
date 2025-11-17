use crate::migrations::Migrator;
use dotenvy::dotenv;
use sea_orm::{Database as SeaDatabase, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use std::env;

pub struct Database {
  pub connection: DatabaseConnection,
}

impl Database {
  pub async fn connect() -> Result<Self, DbErr> {
    dotenv().ok();

    println!("db url: {:?}", env::var("DATABASE_URL").unwrap());

    let db_url = env::var("DATABASE_URL")
      .unwrap_or_else(|_| "sqlite://book_lending.db?mode=rwc".to_string());

    let connection = SeaDatabase::connect(&db_url).await?;

    Ok(Self { connection })
  }

  pub async fn migrate(&self) -> Result<(), DbErr> {
    Migrator::up(&self.connection, None).await
  }
}
