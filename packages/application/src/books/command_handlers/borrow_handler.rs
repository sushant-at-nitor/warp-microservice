use std::sync::Arc;

use async_trait::async_trait;
use common::AppError;
use domain::{aggregate::book::Book, commands::BorrowBookCommand};

use crate::books::BookService;

#[async_trait]
pub trait BorrowBookCommandHandlerTrait: Send + Sync {
  async fn handle(&self, cmd: BorrowBookCommand) -> Result<Book, AppError>;
}

pub struct BorrowBookCommandHandler<T: BookService> {
  service: Arc<T>,
}

impl<T: BookService> BorrowBookCommandHandler<T> {
  pub fn new(service: Arc<T>) -> Self {
    Self { service }
  }
}

#[async_trait]
impl<T: BookService> BorrowBookCommandHandlerTrait
  for BorrowBookCommandHandler<T>
{
  async fn handle(&self, cmd: BorrowBookCommand) -> Result<Book, AppError> {
    self.service.borrow_book(cmd.book_id, cmd.member_id).await
  }
}
