use std::sync::Arc;

use async_trait::async_trait;
use common::AppError;
use domain::aggregate::book::Book;

use crate::books::BookService;

#[async_trait]
pub trait GetAvailableBooksQueryHandlerTrait: Send + Sync {
  async fn handle(&self) -> Result<Vec<Book>, AppError>;
}

pub struct GetAvailableBooksQueryHandler<T: BookService> {
  service: Arc<T>,
}

impl<T: BookService> GetAvailableBooksQueryHandler<T> {
  pub fn new(service: Arc<T>) -> Self {
    Self { service }
  }
}

#[async_trait]
impl<T: BookService> GetAvailableBooksQueryHandlerTrait
  for GetAvailableBooksQueryHandler<T>
{
  async fn handle(&self) -> Result<Vec<Book>, AppError> {
    self
      .service
      .get_available()
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }
}
