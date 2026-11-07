use std::{fs, path::Path};

use crate::field_def::FieldDef;
use crate::helpers::{to_pascal_case, to_snake_case};

pub fn generate(entity: &str, fields: &[FieldDef], output_dir: &Path) {
  let pascal = to_pascal_case(entity);
  let snake = to_snake_case(&pascal);

  // --- Entity file ---
  let model_fields: String = if fields.is_empty() {
    "  // TODO: add fields\n".into()
  } else {
    fields
      .iter()
      .map(|f| format!("  pub {}: {},\n", f.name, f.ty.rust_type(f.nullable)))
      .collect()
  };

  let entity_content = format!(
    r#"use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "{snake}s")]
pub struct Model {{
  #[sea_orm(primary_key)]
  pub id: i32,
{model_fields}}}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {{}}

impl ActiveModelBehavior for ActiveModel {{}}
"#,
    snake = snake,
    model_fields = model_fields
  );

  // --- Mapper file ---
  let model_to_agg_fields: String = if fields.is_empty() {
    "      // TODO: map fields\n".into()
  } else {
    fields
      .iter()
      .map(|f| format!("      {}: model.{},\n", f.name, f.name))
      .collect()
  };

  let agg_to_active_fields: String = if fields.is_empty() {
    "      // TODO: set active model fields\n".into()
  } else {
    fields
      .iter()
      .map(|f| format!("      {}: ActiveValue::Set(item.{}),\n", f.name, f.name))
      .collect()
  };

  let mapper_content = format!(
    r#"use crate::entities::{snake};
use domain::aggregate::{snake}::{pascal};
use sea_orm::ActiveValue;

impl From<{snake}::Model> for {pascal} {{
  fn from(model: {snake}::Model) -> Self {{
    {pascal} {{
      id: model.id,
{model_to_agg_fields}    }}
  }}
}}

impl From<{pascal}> for {snake}::ActiveModel {{
  fn from(item: {pascal}) -> Self {{
    {snake}::ActiveModel {{
      id: if item.id == 0 {{
        ActiveValue::not_set()
      }} else {{
        ActiveValue::Set(item.id)
      }},
{agg_to_active_fields}    }}
  }}
}}
"#,
    snake = snake,
    pascal = pascal,
    model_to_agg_fields = model_to_agg_fields,
    agg_to_active_fields = agg_to_active_fields
  );

  // --- Repository (full CRUD + paginated list) ---
  let repository = format!(
    r#"use crate::entities::{snake};
use common::Paged;
use sea_orm::{{
  ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult,
  EntityTrait, PaginatorTrait, QuerySelect,
}};

pub struct {pascal}Repository {{
  db: DatabaseConnection,
}}

impl {pascal}Repository {{
  pub fn new(db: DatabaseConnection) -> Self {{
    Self {{ db }}
  }}

  pub async fn find_all(&self) -> Result<Vec<{snake}::Model>, DbErr> {{
    {snake}::Entity::find().all(&self.db).await
  }}

  pub async fn list_{snake}(
    &self,
    page: u64,
    page_size: u64,
  ) -> Result<Paged<{snake}::Model>, DbErr> {{
    let page = page.max(1);
    let page_size = page_size.clamp(1, 100);
    let offset = (page - 1) * page_size;
    let total = {snake}::Entity::find().count(&self.db).await?;
    let items = {snake}::Entity::find()
      .offset(offset)
      .limit(page_size)
      .all(&self.db)
      .await?;
    let total_pages = total.div_ceil(page_size);
    Ok(Paged {{ items, total, page, page_size, total_pages }})
  }}

  pub async fn find_by_id(&self, id: i32) -> Result<Option<{snake}::Model>, DbErr> {{
    {snake}::Entity::find_by_id(id).one(&self.db).await
  }}

  pub async fn insert(
    &self,
    model: {snake}::ActiveModel,
  ) -> Result<{snake}::Model, DbErr> {{
    model.insert(&self.db).await
  }}

  pub async fn update(
    &self,
    model: {snake}::ActiveModel,
  ) -> Result<{snake}::Model, DbErr> {{
    model.update(&self.db).await
  }}

  pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {{
    {snake}::Entity::delete_by_id(id).exec(&self.db).await
  }}
}}
"#,
    pascal = pascal,
    snake = snake
  );

  // --- Service impl stub ---
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

  // --- Write all files ---
  let entity_dir = output_dir.join("packages/infrastructure/src/entities");
  let mapper_dir = output_dir.join("packages/infrastructure/src/mappers");
  let repo_dir = output_dir.join("packages/infrastructure/src/repositories");
  let svc_dir = output_dir.join("packages/infrastructure/src/services");

  fs::create_dir_all(&entity_dir).unwrap();
  fs::create_dir_all(&mapper_dir).unwrap();
  fs::create_dir_all(&repo_dir).unwrap();
  fs::create_dir_all(&svc_dir).unwrap();

  fs::write(entity_dir.join(format!("{}.rs", snake)), entity_content).unwrap();
  fs::write(mapper_dir.join(format!("{}.rs", snake)), mapper_content).unwrap();
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
    "  [infra] packages/infrastructure/src/entities/{snake}.rs",
    snake = snake
  );
  println!(
    "  [infra] packages/infrastructure/src/mappers/{snake}.rs",
    snake = snake
  );
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
    generate("member", &[], &tmp);
    let content = fs::read_to_string(
      tmp
        .join("packages/infrastructure/src/repositories/member_repository.rs"),
    )
    .unwrap();
    assert!(content.contains("pub struct MemberRepository"));
    fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn generates_full_repository_with_all_crud_methods() {
    let tmp = env::temp_dir().join("codegen_v2_infra_repo");
    generate("member", &[], &tmp);
    let content = fs::read_to_string(
      tmp.join("packages/infrastructure/src/repositories/member_repository.rs"),
    )
    .unwrap();
    for method in &["find_all", "find_by_id", "insert", "update", "delete", "list_member"] {
      assert!(content.contains(method), "must contain method: {}", method);
    }
    assert!(content.contains("Paged"), "list must return Paged<T>");
    assert!(content.contains("div_ceil"), "pagination must use div_ceil");
    fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn generates_seaorm_entity_with_model_fields() {
    use crate::field_def::{FieldDef, FieldType};
    let tmp = std::env::temp_dir().join("codegen_v2_infra_entity");
    let fields = vec![
      FieldDef { name: "username".into(), ty: FieldType::Str, unique: false, nullable: false },
      FieldDef { name: "score".into(),    ty: FieldType::I32, unique: false, nullable: true  },
    ];
    generate("member", &fields, &tmp);
    let content = std::fs::read_to_string(
      tmp.join("packages/infrastructure/src/entities/member.rs")
    ).unwrap();
    assert!(content.contains("DeriveEntityModel"), "must derive DeriveEntityModel");
    assert!(content.contains("pub username: String"), "must have username field");
    assert!(content.contains("pub score: Option<i32>"), "nullable field must be Option");
    std::fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn generates_mapper_with_both_from_impls() {
    use crate::field_def::{FieldDef, FieldType};
    let tmp = std::env::temp_dir().join("codegen_v2_infra_mapper");
    let fields = vec![
      FieldDef { name: "username".into(), ty: FieldType::Str, unique: false, nullable: false },
    ];
    generate("member", &fields, &tmp);
    let content = std::fs::read_to_string(
      tmp.join("packages/infrastructure/src/mappers/member.rs")
    ).unwrap();
    assert!(content.contains("impl From<member::Model> for Member"), "must have Model->Aggregate impl");
    assert!(content.contains("impl From<Member> for member::ActiveModel"), "must have Aggregate->ActiveModel impl");
    assert!(content.contains("not_set()"), "id==0 must use not_set()");
    assert!(content.contains("ActiveValue::Set(item.username)"), "field must be wrapped in Set");
    std::fs::remove_dir_all(&tmp).ok();
  }
}
