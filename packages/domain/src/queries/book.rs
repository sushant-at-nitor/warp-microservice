use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBookByIdQuery {
  pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllBooksQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBooks {
  #[serde(default)]
  pub page: Option<u64>,
  #[serde(default)]
  pub page_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAvailableBooksQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBookByYearQuery {
  pub year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookQuery {
  pub year: Option<i32>,
  pub name: Option<String>,
  pub author: Option<String>,
  pub isbn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchBooksQuery {
  pub query: BookQuery,
}
