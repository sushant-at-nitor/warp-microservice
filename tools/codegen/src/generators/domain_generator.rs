use crate::sql_parser::TableSchema;
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct TemplateContext {
    aggregate_name: String,
    fields: Vec<TemplateField>,
}

#[derive(Serialize)]
struct TemplateField {
    name_snake: String,
    name_camel: String,
    name_pascal: String,
    ty: String,
    is_pk: bool,
}

pub fn generate(name: &str, schema: TableSchema) -> anyhow::Result<String> {
    let fields = schema
        .fields
        .into_iter()
        .map(|f| TemplateField {
            name_snake: f.name.snake,
            name_camel: f.name.camel,
            name_pascal: f.name.pascal,
            ty: f.ty,
            is_pk: f.is_pk,
        })
        .collect();

    let ctx = TemplateContext {
        aggregate_name: schema.name.pascal,
        fields,
    };

    let mut hbs = Handlebars::new();
    hbs.register_template_string("domain", DOMAIN_TEMPLATE)?;

    let output = hbs.render("domain", &ctx)?;
    Ok(output)
}

const DOMAIN_TEMPLATE: &str = r#"
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{aggregate_name}} {
{{#each fields}}
    pub {{name_snake}}: {{ty}},
{{/each}}
}

impl {{aggregate_name}} {
    pub fn new(
{{#each fields}}
        {{name_snake}}: {{ty}},
{{/each}}
    ) -> Self {
        Self {
{{#each fields}}
            {{name_snake}},
{{/each}}
        }
    }

    pub fn id(&self) -> {{#each fields}}{{#if is_pk}}{{ty}}{{/if}}{{/each}} {
{{#each fields}}{{#if is_pk}}        self.{{name_snake}}{{/if}}{{/each}}
    }
}
"#;
