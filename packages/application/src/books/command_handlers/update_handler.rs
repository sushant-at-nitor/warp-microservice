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
    // let record = self.service.get_by_id(cmd.id).await.map_err(|_| {
    //   AppError::DatabaseError("Not able to fetch the Book".into())
    // })?;

    // let book = record
    //   .ok_or_else(|| AppError::ValidationError("Book not found".into()))?;

    let mut book = self
      .service
      .get_by_id(cmd.id)
      .await
      .map_err(|_| {
        AppError::DatabaseError("Not able to fetch the Book".into())
      })?
      .ok_or_else(|| AppError::ValidationError("Book not found".into()))?;

    book.apply_update(&cmd);

    self
      .service
      .update(book)
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))

    // let result = self.service.get_by_id(cmd.id).await;

    // match result {
    //   Ok(record) => match record {
    //     Some(book) => self
    //       .service
    //       .update(book)
    //       .await
    //       .map_err(|e| AppError::DatabaseError(e.to_string())),
    //     None => Err(AppError::ValidationError("Book not found".into())),
    //   },
    //   Err(_) => {
    //     Err(AppError::DatabaseError("Not able to fetch the Book".into()))
    //   }
    // }
  }
}
