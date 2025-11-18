use std::sync::Arc;

use async_trait::async_trait;
use common::app_error::AppError;
use domain::{aggregate::book::Book, queries::GetBookByIdQuery};

use crate::books::BookService;

#[async_trait]
pub trait GetBookByIdQueryHandlerTrait: Send + Sync {
  async fn handle(
    &self,
    query: GetBookByIdQuery,
  ) -> Result<Option<Book>, AppError>;
}

pub struct GetBookByIdQueryHandler<T: BookService> {
  service: Arc<T>,
}

impl<T: BookService> GetBookByIdQueryHandler<T> {
  pub fn new(service: Arc<T>) -> Self {
    Self { service }
  }
}

#[async_trait]
impl<T: BookService> GetBookByIdQueryHandlerTrait
  for GetBookByIdQueryHandler<T>
{
  async fn handle(
    &self,
    query: GetBookByIdQuery,
  ) -> Result<Option<Book>, AppError> {
    self
      .service
      .get_by_id(query.id)
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }
}
