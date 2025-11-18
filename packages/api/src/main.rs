pub mod endpoints;
pub mod startup;

use env_logger::Env;
use log::info;
use std::sync::Arc;

#[tokio::main]
async fn main() {
  // Load .env file
  dotenvy::dotenv().ok();

  env_logger::Builder::from_env(Env::default().default_filter_or("info"))
    .init();

  info!(
    "Logger initialized with RUST_LOG={}",
    std::env::var("RUST_LOG").unwrap_or_default()
  );

  let state = startup::build_app_state().await;
  let api = endpoints::register(Arc::new(state));

  let port = std::env::var("PORT").unwrap_or_else(|_| "3030".to_string());
  info!("🚀 API running at http://localhost:{}", &port);

  warp::serve(api)
    .run(([127, 0, 0, 1], port.parse().unwrap_or(3030)))
    .await;
}
