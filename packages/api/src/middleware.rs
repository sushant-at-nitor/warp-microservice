use uuid::Uuid;
use warp::Filter;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Extracts or generates a request correlation ID and logs it.
/// Returns the ID string for downstream filters (e.g., to include in responses).
pub fn with_request_id(
) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
  warp::header::optional::<String>(REQUEST_ID_HEADER).map(
    |maybe_id: Option<String>| {
      let id = maybe_id.unwrap_or_else(|| Uuid::new_v4().to_string());
      log::debug!("[request-id] {}", id);
      id
    },
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn generates_id_when_header_absent() {
    let filter = with_request_id();
    let res = warp::test::request().filter(&filter).await.unwrap();
    assert!(!res.is_empty(), "should generate a UUID");
  }

  #[tokio::test]
  async fn passes_through_existing_id() {
    let filter = with_request_id();
    let res = warp::test::request()
      .header(REQUEST_ID_HEADER, "my-trace-id")
      .filter(&filter)
      .await
      .unwrap();
    assert_eq!(res, "my-trace-id");
  }
}
