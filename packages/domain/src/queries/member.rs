use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllMemberQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMemberByIdQuery {
  pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMember {
  #[serde(default)]
  pub page: Option<u64>,
  #[serde(default)]
  pub page_size: Option<u64>,
}
