use crate::entities::book;
use domain::aggregate::BookAggregate;
use sea_orm::ActiveValue;

impl From<book::Model> for BookAggregate {
  fn from(model: book::Model) -> Self {
    BookAggregate {
      id: model.id,
      title: model.title,
      author: model.author,
      isbn: model.isbn,
      available_copies: model.total_copies,
      total_copies: model.total_copies,
      published_year: model.published_year,
      domain_events: vec![], // DB doesn’t store domain events
    }
  }
}

impl From<BookAggregate> for book::ActiveModel {
  fn from(book: BookAggregate) -> Self {
    book::ActiveModel {
      id: ActiveValue::not_set(), // auto increment
      title: ActiveValue::Set(book.title),
      author: ActiveValue::Set(book.author),
      isbn: ActiveValue::Set(book.isbn),
      published_year: ActiveValue::set(book.published_year),
      total_copies: ActiveValue::set(book.total_copies),
      available_copies: ActiveValue::set(book.available_copies),
    }
  }
}
