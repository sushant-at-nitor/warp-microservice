use std::sync::Arc;

use async_trait::async_trait;
use common::app_error::AppError;
use domain::commands::DeleteBookCommand;

use crate::books::BookService;

#[async_trait]
pub trait DeleteBookCommandHandlerTrait: Send + Sync {
  async fn handle(
    &self,
    cmd: DeleteBookCommand,
  ) -> Result<Option<i32>, AppError>;
}

pub struct DeleteBookCommandHandler<T: BookService> {
  service: Arc<T>,
}

impl<T: BookService> DeleteBookCommandHandler<T> {
  pub fn new(service: Arc<T>) -> Self {
    Self { service }
  }
}

#[async_trait]
impl<T: BookService> DeleteBookCommandHandlerTrait
  for DeleteBookCommandHandler<T>
{
  async fn handle(
    &self,
    cmd: DeleteBookCommand,
  ) -> Result<Option<i32>, AppError> {
    self
      .service
      .delete(cmd.id)
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }
}
