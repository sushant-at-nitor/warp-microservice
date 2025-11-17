use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{commands::UpdateBookCommand, events::DomainEvent};

/// Domain events for Book aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookBorrowed {
  book_id: i32,
  member_id: i32,
  occurred_on: DateTime<Utc>,
}

impl DomainEvent for BookBorrowed {
  fn occurred_on(&self) -> DateTime<Utc> {
    self.occurred_on
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookReturned {
  book_id: i32,
  member_id: i32,
  occurred_on: DateTime<Utc>,
}

impl DomainEvent for BookReturned {
  fn occurred_on(&self) -> DateTime<Utc> {
    self.occurred_on
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BookEvent {
  Borrowed(BookBorrowed),
  Returned(BookReturned),
}

// ---------- Aggregate Root ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
  pub id: i32,
  pub title: String,
  pub author: String,
  pub isbn: String,
  pub total_copies: i32,
  pub available_copies: i32,
  pub published_year: Option<i32>,
  pub domain_events: Vec<BookEvent>,
}

impl Book {
  pub fn new(
    id: i32,
    title: String,
    author: String,
    isbn: String,
    total_copies: i32,
    available_copies: i32,
    published_year: Option<i32>,
  ) -> Self {
    Self {
      id,
      title,
      author,
      isbn,
      total_copies,
      available_copies,
      published_year,
      domain_events: vec![],
    }
  }

  pub fn apply_update(&mut self, cmd: &UpdateBookCommand) {
    if let Some(ref title) = cmd.title {
      self.title = title.clone();
    }
    if let Some(ref author) = cmd.author {
      self.author = author.clone();
    }
    if let Some(year) = cmd.published_year {
      self.published_year = Some(year);
    }
    if let Some(ref isbn) = cmd.isbn {
      self.isbn = isbn.clone();
    }
    if let Some(total_copies) = cmd.total_copies {
      self.total_copies = total_copies;
    }
  }

  pub fn borrow_copy(&mut self, member_id: i32) -> Result<(), &str> {
    if self.available_copies == 0 {
      return Err("No copies available");
    }
    self.available_copies -= 1;

    let event = BookBorrowed {
      book_id: self.id,
      member_id,
      occurred_on: Utc::now(),
    };

    self.domain_events.push(BookEvent::Borrowed(event));

    Ok(())
  }

  pub fn return_copy(&mut self, member_id: i32) {
    self.available_copies += 1;

    let event = BookReturned {
      book_id: self.id,
      member_id,
      occurred_on: Utc::now(),
    };

    self.domain_events.push(BookEvent::Returned(event));
  }

  /// Retrieve and clear domain events
  pub fn take_events(&mut self) -> Vec<BookEvent> {
    std::mem::take(&mut self.domain_events)
  }
}
