use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMemberCommand {
  pub name: String,
  pub email: String,
  pub age: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemberCommand {
  #[serde(default)]
  pub id: i32,
  pub name: Option<String>,
  pub email: Option<String>,
  pub age: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMemberCommand {
  pub id: i32,
}
