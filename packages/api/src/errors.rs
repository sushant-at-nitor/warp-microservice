use common::AppError;

#[derive(Debug)]
pub enum ApiError {
  NotFound(String),
  Validation(String),
  Internal(String),
}

impl warp::reject::Reject for ApiError {}

impl From<AppError> for ApiError {
  fn from(e: AppError) -> Self {
    match e {
      AppError::NotFound(msg) => ApiError::NotFound(msg),
      AppError::ValidationError(msg) => ApiError::Validation(msg),
      AppError::DatabaseError(msg) => ApiError::Internal(msg),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn not_found_maps_correctly() {
    let api_err = ApiError::from(AppError::NotFound("Book not found".into()));
    assert!(matches!(api_err, ApiError::NotFound(_)));
  }

  #[test]
  fn validation_maps_correctly() {
    let api_err =
      ApiError::from(AppError::ValidationError("Invalid".into()));
    assert!(matches!(api_err, ApiError::Validation(_)));
  }

  #[test]
  fn database_maps_to_internal() {
    let api_err =
      ApiError::from(AppError::DatabaseError("DB error".into()));
    assert!(matches!(api_err, ApiError::Internal(_)));
  }
}
