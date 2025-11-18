use std::sync::Arc;

use async_trait::async_trait;
use common::app_error::AppError;
use domain::{aggregate::book::Book, queries::GetAllBooksQuery};

use crate::books::BookService;

#[async_trait]
pub trait GetAllBooksQueryHandlerTrait: Send + Sync {
  async fn handle(
    &self,
    query: GetAllBooksQuery,
  ) -> Result<Vec<Book>, AppError>;
}

pub struct GetAllBooksQueryHandler<T: BookService> {
  service: Arc<T>,
}

impl<T: BookService> GetAllBooksQueryHandler<T> {
  pub fn new(service: Arc<T>) -> Self {
    Self { service }
  }
}

#[async_trait]
impl<T: BookService> GetAllBooksQueryHandlerTrait
  for GetAllBooksQueryHandler<T>
{
  async fn handle(&self, _: GetAllBooksQuery) -> Result<Vec<Book>, AppError> {
    self
      .service
      .get_all()
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }
}
