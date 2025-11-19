use crate::sql_parser::{Field, TableSchema};
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct CommandField {
  name: String,
  ty: String,
  is_pk: bool,
}

#[derive(Serialize)]
struct CommandContext {
  aggregate: String,
  create_fields: Vec<CommandField>,
  update_fields: Vec<CommandField>,
  pk_type: String,
}

pub fn generate(schema: TableSchema) -> anyhow::Result<String> {
  let mut pk_field: Option<&Field> = None;

  for f in &schema.fields {
    if f.is_pk {
      pk_field = Some(f);
      break;
    }
  }

  let pk_field =
    pk_field.ok_or_else(|| anyhow::anyhow!("No primary key found"))?;

  let create_fields: Vec<CommandField> = schema
    .fields
    .iter()
    .filter(|f| !f.is_pk)
    .map(|f| CommandField {
      name: f.name.snake.clone(),
      ty: f.ty.clone(),
      is_pk: false,
    })
    .collect();

  let update_fields: Vec<CommandField> = schema
    .fields
    .iter()
    .map(|f| CommandField {
      name: f.name.snake.clone(),
      ty: f.ty.clone(),
      is_pk: f.is_pk,
    })
    .collect();

  let ctx = CommandContext {
    aggregate: schema.name.pascal.clone(),
    create_fields,
    update_fields,
    pk_type: pk_field.ty.clone(),
  };

  let mut hbs = Handlebars::new();
  hbs.register_template_string("commands", COMMAND_TEMPLATE)?;

  Ok(hbs.render("commands", &ctx)?)
}

const COMMAND_TEMPLATE: &str = r#"
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Create{{aggregate}}Command {
{{#each create_fields}}
    pub {{name}}: {{ty}},
{{/each}}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update{{aggregate}}Command {
    pub id: {{pk_type}},
{{#each update_fields}}
    {{#unless is_pk}}
    pub {{name}}: Option<{{ty}}>,
    {{/unless}}
{{/each}}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delete{{aggregate}}Command {
    pub id: {{pk_type}},
}
"#;
