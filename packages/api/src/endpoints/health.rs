use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use warp::Filter;

async fn handle() -> Result<impl warp::Reply, warp::Rejection> {
  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();
  Ok(warp::reply::json(&json!({
    "status": "ok",
    "version": env!("CARGO_PKG_VERSION"),
    "timestamp": timestamp,
  })))
}

pub fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("health").and(warp::get()).and_then(handle)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn health_returns_200() {
    let res = warp::test::request()
      .method("GET")
      .path("/health")
      .reply(&register())
      .await;
    assert_eq!(res.status(), 200);
  }

  #[tokio::test]
  async fn health_body_contains_ok() {
    let res = warp::test::request()
      .method("GET")
      .path("/health")
      .reply(&register())
      .await;
    let body: serde_json::Value = serde_json::from_slice(res.body()).unwrap();
    assert_eq!(body["status"], "ok");
    assert!(body["version"].is_string());
  }
}
