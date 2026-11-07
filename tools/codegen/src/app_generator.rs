use std::{fs, path::Path};

use crate::field_def::FieldDef;
use crate::helpers::{to_pascal_case, to_snake_case};

pub fn generate(entity: &str, fields: &[FieldDef], output_dir: &Path) {
  let pascal = to_pascal_case(entity);
  let snake = to_snake_case(&pascal);

  // ── service.rs ──────────────────────────────────────────────────────────
  let service = format!(
    r#"use async_trait::async_trait;
use common::{{AppError, Paged}};
use domain::aggregate::{snake}::{pascal};

#[async_trait]
pub trait {pascal}Service: Send + Sync {{
  async fn get_all(&self) -> Result<Vec<{pascal}>, AppError>;
  async fn get_by_id(&self, id: i32) -> Result<Option<{pascal}>, AppError>;
  async fn list(&self, page: u64, page_size: u64) -> Result<Paged<{pascal}>, AppError>;
  async fn create(&self, item: {pascal}) -> Result<{pascal}, AppError>;
  async fn update(&self, item: {pascal}) -> Result<{pascal}, AppError>;
  async fn delete(&self, id: i32) -> Result<Option<i32>, AppError>;
}}
"#,
    pascal = pascal,
    snake = snake
  );

  // ── create_handler.rs ────────────────────────────────────────────────────
  let struct_build_lines = if fields.is_empty() {
    "  // TODO: map command fields to aggregate\n".to_string()
  } else {
    let mut lines = "  id: 0,\n".to_string();
    for f in fields {
      lines.push_str(&format!("  {name}: cmd.{name},\n", name = f.name));
    }
    lines
  };

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
  async fn handle(&self, cmd: Create{pascal}Command) -> Result<{pascal}, AppError> {{
    let item = {pascal} {{
{struct_build_lines}    }};
    self.service.create(item).await.map_err(|e| AppError::DatabaseError(e.to_string()))
  }}
}}
"#,
    pascal = pascal,
    snake = snake,
    struct_build_lines = struct_build_lines
  );

  // ── update_handler.rs ────────────────────────────────────────────────────
  let update_apply_lines = if fields.is_empty() {
    "  // TODO: apply optional field updates\n  todo!()".to_string()
  } else {
    fields
      .iter()
      .map(|f| {
        format!(
          "  if let Some(v) = cmd.{name} {{ item.{name} = v; }}",
          name = f.name
        )
      })
      .collect::<Vec<_>>()
      .join("\n")
  };

  let update_handler = format!(
    r#"use std::sync::Arc;
use async_trait::async_trait;
use common::AppError;
use domain::{{aggregate::{snake}::{pascal}, commands::Update{pascal}Command}};
use crate::{snake}::{pascal}Service;

#[async_trait]
pub trait Update{pascal}CommandHandlerTrait: Send + Sync {{
  async fn handle(&self, cmd: Update{pascal}Command) -> Result<{pascal}, AppError>;
}}

pub struct Update{pascal}CommandHandler<T: {pascal}Service> {{
  service: Arc<T>,
}}

impl<T: {pascal}Service> Update{pascal}CommandHandler<T> {{
  pub fn new(service: Arc<T>) -> Self {{ Self {{ service }} }}
}}

#[async_trait]
impl<T: {pascal}Service> Update{pascal}CommandHandlerTrait for Update{pascal}CommandHandler<T> {{
  async fn handle(&self, cmd: Update{pascal}Command) -> Result<{pascal}, AppError> {{
    let mut item = self.service.get_by_id(cmd.id).await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?
      .ok_or_else(|| AppError::NotFound(format!("{pascal} {{}} not found", cmd.id)))?;
{update_apply_lines}
    self.service.update(item).await
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }}
}}
"#,
    pascal = pascal,
    snake = snake,
    update_apply_lines = update_apply_lines
  );

  // ── delete_handler.rs ────────────────────────────────────────────────────
  let delete_handler = format!(
    r#"use std::sync::Arc;
use async_trait::async_trait;
use common::AppError;
use domain::commands::Delete{pascal}Command;
use crate::{snake}::{pascal}Service;

#[async_trait]
pub trait Delete{pascal}CommandHandlerTrait: Send + Sync {{
  async fn handle(&self, cmd: Delete{pascal}Command) -> Result<i32, AppError>;
}}

pub struct Delete{pascal}CommandHandler<T: {pascal}Service> {{
  service: Arc<T>,
}}

impl<T: {pascal}Service> Delete{pascal}CommandHandler<T> {{
  pub fn new(service: Arc<T>) -> Self {{ Self {{ service }} }}
}}

#[async_trait]
impl<T: {pascal}Service> Delete{pascal}CommandHandlerTrait for Delete{pascal}CommandHandler<T> {{
  async fn handle(&self, cmd: Delete{pascal}Command) -> Result<i32, AppError> {{
    self.service.delete(cmd.id).await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?
      .ok_or_else(|| AppError::NotFound(format!("{pascal} {{}} not found", cmd.id)))
  }}
}}
"#,
    pascal = pascal,
    snake = snake
  );

  // ── get_all_handler.rs ───────────────────────────────────────────────────
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

  // ── get_by_id_handler.rs ─────────────────────────────────────────────────
  let get_by_id_handler = format!(
    r#"use std::sync::Arc;
use async_trait::async_trait;
use common::AppError;
use domain::{{aggregate::{snake}::{pascal}, queries::Get{pascal}ByIdQuery}};
use crate::{snake}::{pascal}Service;

#[async_trait]
pub trait Get{pascal}ByIdQueryHandlerTrait: Send + Sync {{
  async fn handle(&self, query: Get{pascal}ByIdQuery) -> Result<{pascal}, AppError>;
}}

pub struct Get{pascal}ByIdQueryHandler<T: {pascal}Service> {{
  service: Arc<T>,
}}

impl<T: {pascal}Service> Get{pascal}ByIdQueryHandler<T> {{
  pub fn new(service: Arc<T>) -> Self {{ Self {{ service }} }}
}}

#[async_trait]
impl<T: {pascal}Service> Get{pascal}ByIdQueryHandlerTrait for Get{pascal}ByIdQueryHandler<T> {{
  async fn handle(&self, query: Get{pascal}ByIdQuery) -> Result<{pascal}, AppError> {{
    self.service.get_by_id(query.id).await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?
      .ok_or_else(|| AppError::NotFound(format!("{pascal} {{}} not found", query.id)))
  }}
}}
"#,
    pascal = pascal,
    snake = snake
  );

  // ── list_handler.rs ──────────────────────────────────────────────────────
  let list_handler = format!(
    r#"use std::sync::Arc;
use async_trait::async_trait;
use common::{{AppError, Paged}};
use domain::{{aggregate::{snake}::{pascal}, queries::List{pascal}Query}};
use crate::{snake}::{pascal}Service;

#[async_trait]
pub trait List{pascal}QueryHandlerTrait: Send + Sync {{
  async fn handle(&self, query: List{pascal}Query) -> Result<Paged<{pascal}>, AppError>;
}}

pub struct List{pascal}QueryHandler<T: {pascal}Service> {{
  service: Arc<T>,
}}

impl<T: {pascal}Service> List{pascal}QueryHandler<T> {{
  pub fn new(service: Arc<T>) -> Self {{ Self {{ service }} }}
}}

#[async_trait]
impl<T: {pascal}Service> List{pascal}QueryHandlerTrait for List{pascal}QueryHandler<T> {{
  async fn handle(&self, query: List{pascal}Query) -> Result<Paged<{pascal}>, AppError> {{
    let page      = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    self.service.list(page, page_size).await
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }}
}}
"#,
    pascal = pascal,
    snake = snake
  );

  // ── write files ──────────────────────────────────────────────────────────
  let app_dir = output_dir
    .join("packages/application/src")
    .join(&snake);
  let cmd_dir = app_dir.join("command_handlers");
  let qry_dir = app_dir.join("query_handlers");

  fs::create_dir_all(&cmd_dir).unwrap();
  fs::create_dir_all(&qry_dir).unwrap();

  fs::write(app_dir.join("service.rs"), service).unwrap();
  fs::write(cmd_dir.join("create_handler.rs"), create_handler).unwrap();
  fs::write(cmd_dir.join("update_handler.rs"), update_handler).unwrap();
  fs::write(cmd_dir.join("delete_handler.rs"), delete_handler).unwrap();
  fs::write(qry_dir.join("get_all_handler.rs"), get_all_handler).unwrap();
  fs::write(qry_dir.join("get_by_id_handler.rs"), get_by_id_handler).unwrap();
  fs::write(qry_dir.join("list_handler.rs"), list_handler).unwrap();

  println!(
    "  [application] packages/application/src/{snake}/service.rs",
    snake = snake
  );
  println!(
    "  [application] packages/application/src/{snake}/command_handlers/create_handler.rs",
    snake = snake
  );
  println!(
    "  [application] packages/application/src/{snake}/command_handlers/update_handler.rs",
    snake = snake
  );
  println!(
    "  [application] packages/application/src/{snake}/command_handlers/delete_handler.rs",
    snake = snake
  );
  println!(
    "  [application] packages/application/src/{snake}/query_handlers/get_all_handler.rs",
    snake = snake
  );
  println!(
    "  [application] packages/application/src/{snake}/query_handlers/get_by_id_handler.rs",
    snake = snake
  );
  println!(
    "  [application] packages/application/src/{snake}/query_handlers/list_handler.rs",
    snake = snake
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::{env, fs};

  #[test]
  fn generates_service_trait() {
    let tmp = env::temp_dir().join("codegen_test_app");
    generate("member", &[], &tmp);
    let content = fs::read_to_string(
      tmp.join("packages/application/src/member/service.rs"),
    )
    .unwrap();
    assert!(content.contains("pub trait MemberService"));
    fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn generates_all_seven_handler_files() {
    use crate::field_def::{FieldDef, FieldType};
    let tmp = env::temp_dir().join("codegen_v2_app_all");
    let fields = vec![
      FieldDef { name: "title".into(), ty: FieldType::Str, unique: false, nullable: false },
    ];
    generate("post", &fields, &tmp);
    let base = tmp.join("packages/application/src/post");
    for f in &[
      "command_handlers/create_handler.rs",
      "command_handlers/update_handler.rs",
      "command_handlers/delete_handler.rs",
      "query_handlers/get_all_handler.rs",
      "query_handlers/get_by_id_handler.rs",
      "query_handlers/list_handler.rs",
      "service.rs",
    ] {
      assert!(base.join(f).exists(), "missing: {}", f);
    }
    fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn update_handler_contains_field_apply_loop() {
    use crate::field_def::{FieldDef, FieldType};
    let tmp = env::temp_dir().join("codegen_v2_app_update");
    let fields = vec![
      FieldDef { name: "username".into(), ty: FieldType::Str, unique: false, nullable: false },
      FieldDef { name: "age".into(),      ty: FieldType::I32, unique: false, nullable: false },
    ];
    generate("member", &fields, &tmp);
    let content = fs::read_to_string(
      tmp.join("packages/application/src/member/command_handlers/update_handler.rs"),
    )
    .unwrap();
    assert!(content.contains("cmd.username"), "update_handler must apply username");
    assert!(content.contains("cmd.age"), "update_handler must apply age");
    fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn service_trait_uses_app_error_not_dberr() {
    use crate::field_def::FieldDef;
    let tmp = env::temp_dir().join("codegen_v2_app_svc");
    generate("post", &[] as &[FieldDef], &tmp);
    let content = fs::read_to_string(
      tmp.join("packages/application/src/post/service.rs"),
    )
    .unwrap();
    assert!(content.contains("AppError"), "service trait must use AppError");
    assert!(!content.contains("DbErr"), "service trait must NOT use DbErr");
    fs::remove_dir_all(&tmp).ok();
  }
}
