use std::sync::Arc;

use async_trait::async_trait;
use common::AppError;
use domain::{aggregate::book::Book, commands::ReturnBookCommand};

use crate::books::BookService;

#[async_trait]
pub trait ReturnBookCommandHandlerTrait: Send + Sync {
  async fn handle(&self, cmd: ReturnBookCommand) -> Result<Book, AppError>;
}

pub struct ReturnBookCommandHandler<T: BookService> {
  service: Arc<T>,
}

impl<T: BookService> ReturnBookCommandHandler<T> {
  pub fn new(service: Arc<T>) -> Self {
    Self { service }
  }
}

#[async_trait]
impl<T: BookService> ReturnBookCommandHandlerTrait
  for ReturnBookCommandHandler<T>
{
  async fn handle(&self, cmd: ReturnBookCommand) -> Result<Book, AppError> {
    self.service.return_book(cmd.book_id, cmd.member_id).await
  }
}
