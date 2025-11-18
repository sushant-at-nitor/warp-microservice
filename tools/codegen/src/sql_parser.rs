use sqlparser::ast::{Statement, TableConstraint};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

use crate::generators::helpers::{
  to_camel_name, to_pascal_name, to_plural_camel_name, to_plural_pascal_name,
  to_plural_screaming_snake_name, to_plural_snake_name,
  to_screaming_snake_name, to_snake_name,
};

#[derive(Debug)]
pub struct NameVariants {
  pub original: String,
  pub snake: String,
  pub camel: String,
  pub pascal: String,
  pub screaming: String,
  pub plural_snake: String,
  pub plural_camel: String,
  pub plural_pascal: String,
  pub plural_screaming: String,
}

impl NameVariants {
  pub fn new(name: &str) -> Self {
    Self {
      original: name.into(),
      snake: to_snake_name(name),
      camel: to_camel_name(name),
      pascal: to_pascal_name(name),
      screaming: to_screaming_snake_name(name),
      plural_snake: to_plural_snake_name(name),
      plural_camel: to_plural_camel_name(name),
      plural_pascal: to_plural_pascal_name(name),
      plural_screaming: to_plural_screaming_snake_name(name),
    }
  }
}

#[derive(Debug)]
pub struct Field {
  pub name: NameVariants,
  pub ty: String,
  pub is_pk: bool,
}

#[derive(Debug)]
pub struct TableSchema {
  pub name: NameVariants,
  pub fields: Vec<Field>,
}

pub fn parse_create_table(sql: &str) -> anyhow::Result<TableSchema> {
  let dialect = MySqlDialect {};
  let ast = Parser::parse_sql(&dialect, sql)?;
  let stmt = ast.first().ok_or_else(|| anyhow::anyhow!("Empty SQL"))?;

  if let Statement::CreateTable(ct) = stmt {
    let table_name_variants = NameVariants::new(&ct.name.to_string());

    let mut pk_columns: Vec<String> = vec![];
    for c in &ct.constraints {
      if let TableConstraint::PrimaryKey { columns, .. } = c {
        pk_columns = columns.iter().map(|c| c.to_string()).collect();
      }
    }

    let fields: Vec<Field> = ct
      .columns
      .iter()
      .map(|col| {
        let orig_name = col.name.to_string();
        let is_pk = pk_columns.contains(&orig_name);

        let name_variants = if is_pk {
          NameVariants::new("id")
        } else {
          NameVariants::new(&orig_name)
        };

        Field {
          name: name_variants,
          ty: sql_to_rust_type(&col.data_type.to_string().to_lowercase()),
          is_pk,
        }
      })
      .collect();

    return Ok(TableSchema {
      name: table_name_variants,
      fields,
    });
  }

  Err(anyhow::anyhow!("Not a CREATE TABLE statement"))
}

fn sql_to_rust_type(sql: &str) -> String {
  match sql {
    s if s.starts_with("int") => "i32".into(),
    s if s.starts_with("tinyint") => "bool".into(),
    "text" => "String".into(),
    _ => "String".into(),
  }
}
