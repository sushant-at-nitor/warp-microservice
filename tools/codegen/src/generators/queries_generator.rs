use crate::sql_parser::{Field, TableSchema};
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct QueryField {
  name: String,
  ty: String,
  is_pk: bool,
}

#[derive(Serialize)]
struct QueryContext {
  aggregate: String,
  aggregate_plural: String,
  pk_ty: String,
  non_pk_fields: Vec<QueryField>,
}

pub fn generate(schema: TableSchema) -> anyhow::Result<String> {
  let pk = schema
    .fields
    .iter()
    .find(|f| f.is_pk)
    .ok_or_else(|| anyhow::anyhow!("No primary key found"))?;

  let non_pk_fields: Vec<QueryField> = schema
    .fields
    .iter()
    .filter(|f| !f.is_pk)
    .map(|f| QueryField {
      name: f.name.snake.clone(),
      ty: f.ty.clone(),
      is_pk: false,
    })
    .collect();

  let ctx = QueryContext {
    aggregate: schema.name.pascal.clone(),
    aggregate_plural: schema.name.plural_pascal.clone(),
    pk_ty: pk.ty.clone(),
    non_pk_fields,
  };

  let mut hbs = Handlebars::new();
  hbs.register_template_string("queries", QUERY_TEMPLATE)?;

  Ok(hbs.render("queries", &ctx)?)
}

const QUERY_TEMPLATE: &str = r#"
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Get{{aggregate}}ByIdQuery {
    pub id: {{pk_ty}},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAll{{aggregate_plural}}Query;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List{{aggregate_plural}}Query {
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{aggregate}}Query {
{{#each non_pk_fields}}
    pub {{name}}: Option<{{ty}}>,
{{/each}}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Search{{aggregate_plural}}Query {
    pub query: {{aggregate}}Query,
}
"#;
