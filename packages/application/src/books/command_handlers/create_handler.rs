use std::sync::Arc;

use async_trait::async_trait;
use common::app_error::AppError;
use domain::{aggregate::book::Book, commands::CreateBookCommand};

use crate::books::BookService;

#[async_trait]
pub trait CreateBookCommandHandlerTrait: Send + Sync {
  async fn handle(&self, cmd: CreateBookCommand) -> Result<Book, AppError>;
}

pub struct CreateBookCommandHandler<T: BookService> {
  service: Arc<T>,
}

impl<T: BookService> CreateBookCommandHandler<T> {
  pub fn new(service: Arc<T>) -> Self {
    Self { service }
  }
}

#[async_trait]
impl<T: BookService> CreateBookCommandHandlerTrait
  for CreateBookCommandHandler<T>
{
  async fn handle(&self, cmd: CreateBookCommand) -> Result<Book, AppError> {
    let book = Book {
      id: 0, // will be assigned by DB
      title: cmd.title,
      author: cmd.author,
      isbn: cmd.isbn,
      total_copies: cmd.total_copies,
      available_copies: cmd.total_copies,
      published_year: cmd.published_year,
      domain_events: vec![],
    };

    self
      .service
      .create(book)
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }
}
