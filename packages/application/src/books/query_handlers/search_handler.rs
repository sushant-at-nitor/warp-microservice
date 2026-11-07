use std::sync::Arc;

use async_trait::async_trait;
use common::AppError;
use domain::{aggregate::book::Book, queries::BookQuery};

use crate::books::BookService;

#[async_trait]
pub trait SearchBooksQueryHandlerTrait: Send + Sync {
  async fn handle(&self, query: BookQuery) -> Result<Vec<Book>, AppError>;
}

pub struct SearchBooksQueryHandler<T: BookService> {
  service: Arc<T>,
}

impl<T: BookService> SearchBooksQueryHandler<T> {
  pub fn new(service: Arc<T>) -> Self {
    Self { service }
  }
}

#[async_trait]
impl<T: BookService> SearchBooksQueryHandlerTrait
  for SearchBooksQueryHandler<T>
{
  async fn handle(&self, query: BookQuery) -> Result<Vec<Book>, AppError> {
    self.service.search(query).await
  }
}
