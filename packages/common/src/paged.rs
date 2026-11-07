use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Paged<T> {
  pub items: Vec<T>,
  pub total: u64,
  pub page: u64,
  pub page_size: u64,
  pub total_pages: u64,
}
