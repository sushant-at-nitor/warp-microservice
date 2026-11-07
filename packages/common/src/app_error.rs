#[derive(Debug)]
pub enum AppError {
  DatabaseError(String),
  ValidationError(String),
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
}
