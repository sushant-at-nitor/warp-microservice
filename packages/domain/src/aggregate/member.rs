use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub age: Option<i32>,
}

impl Member {
  pub fn new(id: i32, name: String, email: String, age: Option<i32>) -> Self {
    Self { id, name, email, age }
  }
}
