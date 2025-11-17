#[derive(Debug)]
pub enum AppError {
  DatabaseError(String),
  ValidationError(String),
}
