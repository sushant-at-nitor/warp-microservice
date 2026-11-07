use crate::field_def::FieldDef;
use crate::helpers::{to_pascal_case, to_snake_case};
use chrono::Local;
use std::{fs, path::Path};

pub fn generate(entity: &str, fields: &[FieldDef], output_dir: &Path) {
  let pascal = to_pascal_case(entity);
  let snake = to_snake_case(&pascal);
  let table_name = format!("{}s", snake);
  let timestamp = Local::now().format("%Y%m%d%H%M%S");
  let file_name = format!("m{}_create_{}_table.rs", timestamp, snake);

  // Iden enum variants: one per field (PascalCase)
  let iden_variants: String = fields
    .iter()
    .map(|f| format!("  {},\n", to_pascal_case(&f.name)))
    .collect();

  // col defs per field
  let col_defs: String = fields
    .iter()
    .map(|f| {
      let variant = to_pascal_case(&f.name);
      let type_method = f.ty.seaorm_col_method();
      let mut chain = format!(
        "          .col(ColumnDef::new({pascal}::{variant}).{type_method}",
        pascal = pascal,
        variant = variant,
        type_method = type_method
      );
      if !f.nullable {
        chain.push_str(".not_null()");
      }
      if f.unique {
        chain.push_str(".unique_key()");
      }
      chain.push_str(")\n");
      chain
    })
    .collect();

  let content = format!(
    r#"use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {{
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
    manager
      .create_table(
        Table::create()
          .table("{table_name}")
          .if_not_exists()
          .col(
            ColumnDef::new({pascal}::Id)
              .integer()
              .not_null()
              .primary_key()
              .auto_increment(),
          )
{col_defs}          .to_owned(),
      )
      .await
  }}

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
    manager
      .drop_table(Table::drop().table({pascal}::Table).to_owned())
      .await
  }}
}}

#[derive(Iden)]
enum {pascal} {{
  Table,
  Id,
{iden_variants}}}
"#,
    table_name = table_name,
    pascal = pascal,
    col_defs = col_defs,
    iden_variants = iden_variants
  );

  let migration_dir = output_dir.join("packages/database/src/migrations");
  fs::create_dir_all(&migration_dir).unwrap();
  fs::write(migration_dir.join(&file_name), content).unwrap();

  println!(
    "  [database] packages/database/src/migrations/{}",
    file_name
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::field_def::{FieldDef, FieldType};

  #[test]
  fn generates_migration_file() {
    let tmp = std::env::temp_dir().join("codegen_v2_migration");
    let fields = vec![
      FieldDef { name: "username".into(), ty: FieldType::Str, unique: false, nullable: false },
      FieldDef { name: "score".into(),    ty: FieldType::I32, unique: false, nullable: true  },
    ];
    generate("member", &fields, &tmp);
    let dir = tmp.join("packages/database/src/migrations");
    let files: Vec<_> = std::fs::read_dir(&dir)
      .unwrap()
      .filter_map(|e| e.ok())
      .collect();
    assert!(!files.is_empty(), "migration file must be created");
    let content = std::fs::read_to_string(files[0].path()).unwrap();
    assert!(content.contains("Username"), "must include Username Iden variant");
    assert!(content.contains(".string()"), "must include string() col method");
    assert!(content.contains(".not_null()"), "required field must be not_null");
    assert!(content.contains("Score"), "must include Score Iden variant");
    std::fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn unique_field_has_unique_key_in_migration() {
    let tmp = std::env::temp_dir().join("codegen_v2_migration_unique");
    let fields = vec![
      FieldDef { name: "email".into(), ty: FieldType::Str, unique: true, nullable: false },
    ];
    generate("user", &fields, &tmp);
    let dir = tmp.join("packages/database/src/migrations");
    let files: Vec<_> = std::fs::read_dir(&dir)
      .unwrap()
      .filter_map(|e| e.ok())
      .collect();
    let content = std::fs::read_to_string(files[0].path()).unwrap();
    assert!(content.contains(".unique_key()"), "unique field must have unique_key()");
    std::fs::remove_dir_all(&tmp).ok();
  }
}
