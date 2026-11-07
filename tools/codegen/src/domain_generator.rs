use std::{fs, path::Path};

use crate::field_def::FieldDef;
use crate::helpers::{to_pascal_case, to_snake_case};

pub fn generate(entity: &str, fields: &[FieldDef], output_dir: &Path) {
  let pascal = to_pascal_case(entity);
  let snake = to_snake_case(&pascal);

  // struct fields for aggregate (use FieldType::rust_type with nullable)
  let struct_fields: String = if fields.is_empty() {
    "  // TODO: add domain fields\n".into()
  } else {
    fields
      .iter()
      .map(|f| format!("  pub {}: {},\n", f.name, f.ty.rust_type(f.nullable)))
      .collect()
  };

  // fields for CreateCommand — always non-optional (required at creation)
  let create_fields: String = if fields.is_empty() {
    "  // TODO: add fields\n".into()
  } else {
    fields
      .iter()
      .map(|f| format!("  pub {}: {},\n", f.name, f.ty.rust_type(false)))
      .collect()
  };

  // fields for UpdateCommand — all Option<BaseType>
  let update_fields: String = if fields.is_empty() {
    "  // TODO: add optional update fields\n".into()
  } else {
    fields
      .iter()
      .map(|f| format!("  pub {}: Option<{}>,\n", f.name, f.ty.rust_type(false)))
      .collect()
  };

  let aggregate = format!(
    r#"use serde::{{Deserialize, Serialize}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {pascal} {{
  pub id: i32,
{struct_fields}}}

impl {pascal} {{
  pub fn new(id: i32) -> Self {{
    Self {{ id }}
  }}
}}
"#,
    pascal = pascal,
    struct_fields = struct_fields
  );

  let commands = format!(
    r#"use serde::{{Deserialize, Serialize}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Create{pascal}Command {{
{create_fields}}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update{pascal}Command {{
  #[serde(default)]
  pub id: i32,
{update_fields}}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delete{pascal}Command {{
  pub id: i32,
}}
"#,
    pascal = pascal,
    create_fields = create_fields,
    update_fields = update_fields
  );

  let queries = format!(
    r#"use serde::{{Deserialize, Serialize}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAll{pascal}Query;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Get{pascal}ByIdQuery {{
  pub id: i32,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List{pascal} {{
  #[serde(default)]
  pub page: Option<u64>,
  #[serde(default)]
  pub page_size: Option<u64>,
}}
"#,
    pascal = pascal
  );

  let aggregate_dir = output_dir.join("packages/domain/src/aggregate");
  let commands_dir = output_dir.join("packages/domain/src/commands");
  let queries_dir = output_dir.join("packages/domain/src/queries");

  fs::create_dir_all(&aggregate_dir).unwrap();
  fs::create_dir_all(&commands_dir).unwrap();
  fs::create_dir_all(&queries_dir).unwrap();

  fs::write(aggregate_dir.join(format!("{}.rs", snake)), aggregate).unwrap();
  fs::write(commands_dir.join(format!("{}.rs", snake)), commands).unwrap();
  fs::write(queries_dir.join(format!("{}.rs", snake)), queries).unwrap();

  println!(
    "  [domain] packages/domain/src/aggregate/{snake}.rs",
    snake = snake
  );
  println!(
    "  [domain] packages/domain/src/commands/{snake}.rs",
    snake = snake
  );
  println!(
    "  [domain] packages/domain/src/queries/{snake}.rs",
    snake = snake
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn generates_aggregate_with_correct_name() {
    let tmp = env::temp_dir().join("codegen_test_domain_agg");
    generate("member", &[], &tmp);
    let content =
      fs::read_to_string(tmp.join("packages/domain/src/aggregate/member.rs")).unwrap();
    assert!(content.contains("pub struct Member"));
    fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn generates_commands_with_correct_name() {
    let tmp = env::temp_dir().join("codegen_test_domain_cmd");
    generate("book_loan", &[], &tmp);
    let content =
      fs::read_to_string(tmp.join("packages/domain/src/commands/book_loan.rs")).unwrap();
    assert!(content.contains("pub struct CreateBookLoanCommand"));
    fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn generates_queries_with_correct_name() {
    let tmp = env::temp_dir().join("codegen_test_domain_qry");
    generate("member", &[], &tmp);
    let content =
      fs::read_to_string(tmp.join("packages/domain/src/queries/member.rs")).unwrap();
    assert!(content.contains("pub struct GetAllMemberQuery"));
    fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn aggregate_contains_declared_fields() {
    use crate::field_def::{FieldDef, FieldType};
    let tmp = std::env::temp_dir().join("codegen_v2_domain_agg");
    let fields = vec![
      FieldDef {
        name: "username".into(),
        ty: FieldType::Str,
        unique: false,
        nullable: false,
      },
      FieldDef {
        name: "age".into(),
        ty: FieldType::I32,
        unique: false,
        nullable: true,
      },
    ];
    generate("member", &fields, &tmp);
    let content = std::fs::read_to_string(
      tmp.join("packages/domain/src/aggregate/member.rs"),
    )
    .unwrap();
    assert!(content.contains("pub username: String"), "missing username field");
    assert!(content.contains("pub age: Option<i32>"), "missing age field");
    std::fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn create_command_contains_all_fields_non_optional() {
    use crate::field_def::{FieldDef, FieldType};
    let tmp = std::env::temp_dir().join("codegen_v2_domain_cmd");
    let fields = vec![FieldDef {
      name: "title".into(),
      ty: FieldType::Str,
      unique: false,
      nullable: true,
    }];
    generate("post", &fields, &tmp);
    let content = std::fs::read_to_string(
      tmp.join("packages/domain/src/commands/post.rs"),
    )
    .unwrap();
    assert!(
      content.contains("pub title: String"),
      "CreatePostCommand must have non-optional title"
    );
    std::fs::remove_dir_all(&tmp).ok();
  }
}
