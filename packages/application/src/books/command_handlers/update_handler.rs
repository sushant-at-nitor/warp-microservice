use std::sync::Arc;

use async_trait::async_trait;
use common::AppError;
use domain::{aggregate::book::Book, commands::UpdateBookCommand};

use crate::books::BookService;

#[async_trait]
pub trait UpdateBookCommandHandlerTrait: Send + Sync {
  async fn handle(&self, cmd: UpdateBookCommand) -> Result<Book, AppError>;
}

pub struct UpdateBookCommandHandler<T: BookService> {
  service: Arc<T>,
}

impl<T: BookService> UpdateBookCommandHandler<T> {
  pub fn new(service: Arc<T>) -> Self {
    Self { service }
  }
}

#[async_trait]
impl<T: BookService> UpdateBookCommandHandlerTrait
  for UpdateBookCommandHandler<T>
{
  async fn handle(&self, cmd: UpdateBookCommand) -> Result<Book, AppError> {
    let mut book = self
      .service
      .get_by_id(cmd.id)
      .await?
      .ok_or_else(|| AppError::NotFound(format!("Book {} not found", cmd.id)))?;

    book.apply_update(&cmd);

    self.service.update(book).await
  }
}
