use std::{fs, path::Path};

use crate::helpers::{to_pascal_case, to_snake_case};

pub fn generate(entity: &str, output_dir: &Path) {
  let pascal = to_pascal_case(entity);
  let snake = to_snake_case(&pascal);

  let repository = format!(
    r#"use sea_orm::DatabaseConnection;

pub struct {pascal}Repository {{
  db: DatabaseConnection,
}}

impl {pascal}Repository {{
  pub fn new(db: DatabaseConnection) -> Self {{
    Self {{ db }}
  }}
  // TODO: implement find_all, find_by_id, insert, update, delete
}}
"#,
    pascal = pascal
  );

  let service_impl = format!(
    r#"use crate::repositories::{snake}_repository::{pascal}Repository;
use application::{snake}::{pascal}Service;
use async_trait::async_trait;
use common::{{AppError, Paged}};
use domain::aggregate::{snake}::{pascal};
use sea_orm::DbErr;

pub struct {pascal}ServiceImpl {{
  repo: {pascal}Repository,
}}

impl {pascal}ServiceImpl {{
  pub fn new(repo: {pascal}Repository) -> Self {{
    Self {{ repo }}
  }}
}}

#[async_trait]
impl {pascal}Service for {pascal}ServiceImpl {{
  async fn get_all(&self) -> Result<Vec<{pascal}>, DbErr> {{ todo!() }}
  async fn list(&self, _page: u64, _page_size: u64) -> Result<Paged<{pascal}>, DbErr> {{ todo!() }}
  async fn get_by_id(&self, _id: i32) -> Result<Option<{pascal}>, DbErr> {{ todo!() }}
  async fn create(&self, item: {pascal}) -> Result<{pascal}, DbErr> {{ todo!() }}
  async fn update(&self, item: {pascal}) -> Result<{pascal}, DbErr> {{ todo!() }}
  async fn delete(&self, _id: i32) -> Result<Option<i32>, DbErr> {{ todo!() }}
}}
"#,
    pascal = pascal,
    snake = snake
  );

  let repo_dir = output_dir.join("packages/infrastructure/src/repositories");
  let svc_dir = output_dir.join("packages/infrastructure/src/services");

  fs::create_dir_all(&repo_dir).unwrap();
  fs::create_dir_all(&svc_dir).unwrap();

  fs::write(
    repo_dir.join(format!("{}_repository.rs", snake)),
    repository,
  )
  .unwrap();
  fs::write(
    svc_dir.join(format!("{}_service.rs", snake)),
    service_impl,
  )
  .unwrap();

  println!(
    "  [infra] packages/infrastructure/src/repositories/{snake}_repository.rs",
    snake = snake
  );
  println!(
    "  [infra] packages/infrastructure/src/services/{snake}_service.rs",
    snake = snake
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn generates_repository() {
    let tmp = env::temp_dir().join("codegen_test_infra");
    generate("member", &tmp);
    let content = fs::read_to_string(
      tmp
        .join("packages/infrastructure/src/repositories/member_repository.rs"),
    )
    .unwrap();
    assert!(content.contains("pub struct MemberRepository"));
    fs::remove_dir_all(&tmp).ok();
  }
}
