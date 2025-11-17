use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Paged<T> {
  pub items: Vec<T>,
  pub total: u64,
  pub page: u64,
  pub page_size: u64,
  pub total_pages: u64,
}
