use database::Database;

pub struct AppState {
  pub db: Database,
}

pub async fn build_app_state() -> AppState {
  let db = Database::connect()
    .await
    .expect("Not able to connect database");

  match db.migrate().await {
    Ok(_) => println!("✅ Migrations complete!"),
    Err(err) => println!("Error migrating database: {:?}", err),
  }

  AppState { db }
}
