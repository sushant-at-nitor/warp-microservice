use std::sync::Arc;

use async_trait::async_trait;
use common::{Paged, app_error::AppError};
use domain::{aggregate::book::Book, queries::ListBooks};

use crate::books::BookService;

#[async_trait]
pub trait ListBooksQueryHandlerTrait: Send + Sync {
  async fn handle(&self, query: ListBooks) -> Result<Paged<Book>, AppError>;
}

pub struct ListBooksQueryHandler<T: BookService> {
  service: Arc<T>,
}

impl<T: BookService> ListBooksQueryHandler<T> {
  pub fn new(service: Arc<T>) -> Self {
    Self { service }
  }
}

#[async_trait]
impl<T: BookService> ListBooksQueryHandlerTrait for ListBooksQueryHandler<T> {
  async fn handle(&self, pager: ListBooks) -> Result<Paged<Book>, AppError> {
    self
      .service
      .list(pager.page.unwrap_or(1), pager.page_size.unwrap_or(10))
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }
}
