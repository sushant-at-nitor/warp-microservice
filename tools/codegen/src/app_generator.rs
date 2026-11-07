use std::{fs, path::Path};

use crate::helpers::{to_pascal_case, to_snake_case};

pub fn generate(entity: &str, output_dir: &Path) {
  let pascal = to_pascal_case(entity);
  let snake = to_snake_case(&pascal);

  let service = format!(
    r#"use async_trait::async_trait;
use common::{{AppError, Paged}};
use domain::aggregate::{snake}::{pascal};
use sea_orm::DbErr;

#[async_trait]
pub trait {pascal}Service: Send + Sync {{
  async fn get_all(&self) -> Result<Vec<{pascal}>, DbErr>;
  async fn list(&self, page: u64, page_size: u64) -> Result<Paged<{pascal}>, DbErr>;
  async fn get_by_id(&self, id: i32) -> Result<Option<{pascal}>, DbErr>;
  async fn create(&self, item: {pascal}) -> Result<{pascal}, DbErr>;
  async fn update(&self, item: {pascal}) -> Result<{pascal}, DbErr>;
  async fn delete(&self, id: i32) -> Result<Option<i32>, DbErr>;
}}
"#,
    pascal = pascal,
    snake = snake
  );

  let create_handler = format!(
    r#"use std::sync::Arc;
use async_trait::async_trait;
use common::AppError;
use domain::{{aggregate::{snake}::{pascal}, commands::Create{pascal}Command}};
use crate::{snake}::{pascal}Service;

#[async_trait]
pub trait Create{pascal}CommandHandlerTrait: Send + Sync {{
  async fn handle(&self, cmd: Create{pascal}Command) -> Result<{pascal}, AppError>;
}}

pub struct Create{pascal}CommandHandler<T: {pascal}Service> {{
  service: Arc<T>,
}}

impl<T: {pascal}Service> Create{pascal}CommandHandler<T> {{
  pub fn new(service: Arc<T>) -> Self {{ Self {{ service }} }}
}}

#[async_trait]
impl<T: {pascal}Service> Create{pascal}CommandHandlerTrait for Create{pascal}CommandHandler<T> {{
  async fn handle(&self, _cmd: Create{pascal}Command) -> Result<{pascal}, AppError> {{
    // TODO: build {pascal} from cmd and call service.create
    todo!()
  }}
}}
"#,
    pascal = pascal,
    snake = snake
  );

  let get_all_handler = format!(
    r#"use std::sync::Arc;
use async_trait::async_trait;
use common::AppError;
use domain::aggregate::{snake}::{pascal};
use crate::{snake}::{pascal}Service;

#[async_trait]
pub trait GetAll{pascal}QueryHandlerTrait: Send + Sync {{
  async fn handle(&self) -> Result<Vec<{pascal}>, AppError>;
}}

pub struct GetAll{pascal}QueryHandler<T: {pascal}Service> {{
  service: Arc<T>,
}}

impl<T: {pascal}Service> GetAll{pascal}QueryHandler<T> {{
  pub fn new(service: Arc<T>) -> Self {{ Self {{ service }} }}
}}

#[async_trait]
impl<T: {pascal}Service> GetAll{pascal}QueryHandlerTrait for GetAll{pascal}QueryHandler<T> {{
  async fn handle(&self) -> Result<Vec<{pascal}>, AppError> {{
    self.service.get_all().await.map_err(|e| AppError::DatabaseError(e.to_string()))
  }}
}}
"#,
    pascal = pascal,
    snake = snake
  );

  let app_dir = output_dir
    .join("packages/application/src")
    .join(&snake);
  let cmd_dir = app_dir.join("command_handlers");
  let qry_dir = app_dir.join("query_handlers");

  fs::create_dir_all(&cmd_dir).unwrap();
  fs::create_dir_all(&qry_dir).unwrap();

  fs::write(app_dir.join("service.rs"), service).unwrap();
  fs::write(cmd_dir.join("create_handler.rs"), create_handler).unwrap();
  fs::write(qry_dir.join("get_all_handler.rs"), get_all_handler).unwrap();

  println!(
    "  [application] packages/application/src/{snake}/service.rs",
    snake = snake
  );
  println!(
    "  [application] packages/application/src/{snake}/command_handlers/create_handler.rs",
    snake = snake
  );
  println!(
    "  [application] packages/application/src/{snake}/query_handlers/get_all_handler.rs",
    snake = snake
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn generates_service_trait() {
    let tmp = env::temp_dir().join("codegen_test_app");
    generate("member", &tmp);
    let content = fs::read_to_string(
      tmp.join("packages/application/src/member/service.rs"),
    )
    .unwrap();
    assert!(content.contains("pub trait MemberService"));
    fs::remove_dir_all(&tmp).ok();
  }
}
