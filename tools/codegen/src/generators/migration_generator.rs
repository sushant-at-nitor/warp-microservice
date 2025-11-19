use crate::sql_parser::{Field, TableSchema};
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct MigrationField {
  name_pascal: String, // For Iden enum
  name_snake: String,  // For SeaORM column builder
  sea_type: String,    // SeaORM column type (integer(), string(), etc.)
  is_pk: bool,
  is_nullable: bool,
  is_unique: bool,
}

#[derive(Serialize)]
struct MigrationContext {
  table_name: String,
  table_name_plural: String,
  table_pascal: String,
  fields: Vec<MigrationField>,
}

pub fn generate(schema: TableSchema) -> anyhow::Result<String> {
  let mut fields = vec![];

  for f in &schema.fields {
    fields.push(MigrationField {
      name_pascal: f.name.pascal.clone(),
      name_snake: f.name.snake.clone(),
      sea_type: sql_to_sea_type(&f.ty),
      is_pk: f.is_pk,
      is_nullable: !f.is_pk, // simple rule: only PK is not null
      is_unique: false,      // you can extend later
    });
  }

  let ctx = MigrationContext {
    table_name_plural: schema.name.plural_snake.clone(),
    table_name: schema.name.snake.clone(),
    table_pascal: schema.name.pascal.clone(),
    fields,
  };

  let mut hbs = Handlebars::new();
  hbs.register_template_string("migration", MIGRATION_TEMPLATE)?;

  Ok(hbs.render("migration", &ctx)?)
}

fn sql_to_sea_type(rust_ty: &str) -> String {
  match rust_ty {
    "i32" => "integer()".into(),
    "i64" => "big_integer()".into(),
    "bool" => "boolean()".into(),
    "String" => "string()".into(),
    _ => "string()".into(),
  }
}

const MIGRATION_TEMPLATE: &str = r#"
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("{{table_name_plural}}")
                    //.table({{table_pascal}}::Table)
                    .if_not_exists()
                    {{#each fields}}
                    .col(
                        ColumnDef::new({{../table_pascal}}::{{name_pascal}})
                            .{{sea_type}}
                            {{#if is_pk}}.not_null().primary_key().auto_increment(){{else}}
                                {{#if is_nullable}}.null(){{else}}.not_null(){{/if}}
                                {{#if is_unique}}.unique_key(){{/if}}
                            {{/if}}
                    )
                    {{/each}}
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table({{table_pascal}}::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum {{table_pascal}} {
    Table,
    {{#each fields}}
    {{name_pascal}},
    {{/each}}
}
"#;
