use api::{config::Config, endpoints, startup};
use env_logger::Env;
use log::info;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();

  let cfg = Config::from_env();

  env_logger::Builder::from_env(Env::default().default_filter_or(&cfg.rust_log))
    .init();

  info!("Config loaded: port={}, log={}", cfg.port, cfg.rust_log);

  let state = startup::build_app_state().await;
  let api = endpoints::register(Arc::new(state));

  let addr: SocketAddr = ([0, 0, 0, 0], cfg.port).into();
  info!("🚀 API running at http://localhost:{}", cfg.port);

  warp::serve(api)
    .bind(addr)
    .await
    .graceful(async {
      tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
      info!("Shutdown signal received — draining connections");
    })
    .run()
    .await;

  info!("Server stopped.");
}
