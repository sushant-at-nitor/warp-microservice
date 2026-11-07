use serde_json::json;
use warp::{Rejection, Reply, http::StatusCode};

use crate::errors::ApiError;

pub async fn handle_rejection(
  err: Rejection,
) -> Result<impl Reply, Rejection> {
  let (status, message) = if err.is_not_found() {
    (StatusCode::NOT_FOUND, "Not found".to_string())
  } else if let Some(e) = err.find::<ApiError>() {
    match e {
      ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
      ApiError::Validation(msg) => {
        (StatusCode::UNPROCESSABLE_ENTITY, msg.clone())
      }
      ApiError::Internal(msg) => {
        (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
      }
    }
  } else if let Some(e) =
    err.find::<warp::filters::body::BodyDeserializeError>()
  {
    (StatusCode::BAD_REQUEST, format!("Invalid body: {}", e))
  } else {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      "Internal server error".to_string(),
    )
  };

  let body =
    warp::reply::json(&json!({ "error": message, "status": status.as_u16() }));
  Ok(warp::reply::with_status(body, status))
}
