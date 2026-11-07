use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("Database error: {0}")]
  DatabaseError(String),
  #[error("Validation error: {0}")]
  ValidationError(String),
  #[error("Not found: {0}")]
  NotFound(String),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn app_error_not_found_exists() {
    let e = AppError::NotFound("Book 42 not found".into());
    match e {
      AppError::NotFound(msg) => assert_eq!(msg, "Book 42 not found"),
      _ => panic!("wrong variant"),
    }
  }

  #[test]
  fn app_error_implements_display() {
    let e = AppError::ValidationError("bad input".into());
    assert_eq!(e.to_string(), "Validation error: bad input");
  }

  #[test]
  fn app_error_implements_std_error() {
    let e: Box<dyn std::error::Error> =
      Box::new(AppError::DatabaseError("conn fail".into()));
    assert!(e.to_string().contains("conn fail"));
  }
}
