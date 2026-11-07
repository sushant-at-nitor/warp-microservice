/// All configuration read from environment at startup.
/// Panics early with a clear message if a required variable is malformed.
#[derive(Debug, Clone)]
pub struct Config {
  pub port: u16,
  pub database_url: String,
  pub rust_log: String,
}

impl Config {
  pub fn from_env() -> Self {
    let port = std::env::var("PORT")
      .unwrap_or_else(|_| "3030".to_string())
      .parse::<u16>()
      .expect("PORT must be a valid u16 (1–65535)");

    let database_url = std::env::var("DATABASE_URL")
      .unwrap_or_else(|_| "sqlite://./db.sqlite?mode=rwc".to_string());

    let rust_log = std::env::var("RUST_LOG")
      .unwrap_or_else(|_| "info".to_string());

    Self { port, database_url, rust_log }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn defaults_are_sensible() {
    // Clear env so we test defaults
    unsafe {
      std::env::remove_var("PORT");
      std::env::remove_var("DATABASE_URL");
      std::env::remove_var("RUST_LOG");
    }
    let cfg = Config::from_env();
    assert_eq!(cfg.port, 3030);
    assert!(cfg.database_url.contains("sqlite"));
    assert_eq!(cfg.rust_log, "info");
  }
}
