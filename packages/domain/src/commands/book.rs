use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBookCommand {
  pub title: String,
  pub author: String,
  pub isbn: String,
  pub total_copies: i32,
  pub published_year: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBookCommand {
  pub id: i32,
  pub title: Option<String>,
  pub author: Option<String>,
  pub isbn: Option<String>,
  pub total_copies: Option<i32>,
  pub published_year: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBookCommand {
  pub id: i32,
}
