mod api_generator;
mod app_generator;
mod domain_generator;
mod field_def;
mod helpers;
mod infra_generator;
mod migration_generator;
mod type_map;

use clap::Parser;
use field_def::{parse_field, FieldDef};
use helpers::{to_pascal_case, to_snake_case};
use std::path::Path;

#[derive(Parser)]
#[command(name = "codegen", about = "Scaffold a new entity across all 5 layers")]
struct Cli {
  entity: String,
  #[arg(long = "field", value_name = "name:Type[:modifier]")]
  fields: Vec<String>,
}

fn main() {
  let cli = Cli::parse();
  let fields: Vec<FieldDef> = cli
    .fields
    .iter()
    .map(|f| {
      parse_field(f).unwrap_or_else(|e| {
        eprintln!("Error parsing --field '{}': {}", f, e);
        std::process::exit(1);
      })
    })
    .collect();

  let entity = &cli.entity;
  let output_dir = Path::new(".");

  println!("Generating scaffold for: {}", entity);

  domain_generator::generate(entity, &fields, output_dir);
  app_generator::generate(entity, &fields, output_dir);
  infra_generator::generate(entity, &fields, output_dir);
  api_generator::generate(entity, &fields, output_dir);
  migration_generator::generate(entity, &fields, output_dir);

  let pascal = to_pascal_case(entity);
  let snake = to_snake_case(&pascal);
  println!("\n=== REGISTRATION CHECKLIST ===");
  println!(
    "packages/domain/src/lib.rs              → pub mod {snake}; (aggregate, commands, queries)"
  );
  println!(
    "packages/infrastructure/src/entities/mod.rs → pub mod {snake};"
  );
  println!(
    "packages/infrastructure/src/mappers/mod.rs  → pub mod {snake};"
  );
  println!(
    "packages/api/src/endpoints/mod.rs           → register {snake}::register(state.clone())"
  );
  println!(
    "packages/database/src/migrations/mod.rs     → register the generated migration"
  );
  println!("\nDone! Register the files above, then run: cargo build");
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::field_def::{FieldDef, FieldType};
  use std::{env, fs};

  fn sample_fields() -> Vec<FieldDef> {
    vec![
      FieldDef { name: "title".into(),  ty: FieldType::Str,  unique: false, nullable: false },
      FieldDef { name: "price".into(),  ty: FieldType::F64,  unique: false, nullable: true  },
      FieldDef { name: "sku".into(),    ty: FieldType::Str,  unique: true,  nullable: false },
    ]
  }

  #[test]
  fn all_22_files_generated_for_entity_with_fields() {
    let tmp = env::temp_dir().join("codegen_v2_integration");
    fs::remove_dir_all(&tmp).ok(); // clean any leftover from prior run
    let fields = sample_fields();

    domain_generator::generate("product", &fields, &tmp);
    app_generator::generate("product", &fields, &tmp);
    infra_generator::generate("product", &fields, &tmp);
    api_generator::generate("product", &fields, &tmp);
    migration_generator::generate("product", &fields, &tmp);

    // domain: 3 files
    assert!(tmp.join("packages/domain/src/aggregate/product.rs").exists(), "domain aggregate");
    assert!(tmp.join("packages/domain/src/commands/product.rs").exists(), "domain commands");
    assert!(tmp.join("packages/domain/src/queries/product.rs").exists(), "domain queries");

    // app: 7 files
    assert!(tmp.join("packages/application/src/product/service.rs").exists(), "app service");
    assert!(tmp.join("packages/application/src/product/command_handlers/create_handler.rs").exists(), "app create_handler");
    assert!(tmp.join("packages/application/src/product/command_handlers/update_handler.rs").exists(), "app update_handler");
    assert!(tmp.join("packages/application/src/product/command_handlers/delete_handler.rs").exists(), "app delete_handler");
    assert!(tmp.join("packages/application/src/product/query_handlers/get_all_handler.rs").exists(), "app get_all_handler");
    assert!(tmp.join("packages/application/src/product/query_handlers/get_by_id_handler.rs").exists(), "app get_by_id_handler");
    assert!(tmp.join("packages/application/src/product/query_handlers/list_handler.rs").exists(), "app list_handler");

    // api: 7 files
    assert!(tmp.join("packages/api/src/endpoints/product/create.rs").exists(), "api create");
    assert!(tmp.join("packages/api/src/endpoints/product/update.rs").exists(), "api update");
    assert!(tmp.join("packages/api/src/endpoints/product/delete.rs").exists(), "api delete");
    assert!(tmp.join("packages/api/src/endpoints/product/get_all.rs").exists(), "api get_all");
    assert!(tmp.join("packages/api/src/endpoints/product/get_by_id.rs").exists(), "api get_by_id");
    assert!(tmp.join("packages/api/src/endpoints/product/list.rs").exists(), "api list");
    assert!(tmp.join("packages/api/src/endpoints/product/mod.rs").exists(), "api mod");

    // infra: 4 files
    assert!(tmp.join("packages/infrastructure/src/entities/product.rs").exists(), "infra entity");
    assert!(tmp.join("packages/infrastructure/src/mappers/product.rs").exists(), "infra mapper");
    assert!(tmp.join("packages/infrastructure/src/repositories/product_repository.rs").exists(), "infra repository");
    assert!(tmp.join("packages/infrastructure/src/services/product_service.rs").exists(), "infra service");

    // migration: 1 file (timestamp in name — count files in dir)
    let mig_dir = tmp.join("packages/database/src/migrations");
    let mig_files: Vec<_> = fs::read_dir(&mig_dir).unwrap()
      .filter_map(|e| e.ok())
      .filter(|e| e.file_name().to_string_lossy().ends_with("_create_product_table.rs"))
      .collect();
    assert_eq!(mig_files.len(), 1, "migration: expected 1 create_product_table file");

    fs::remove_dir_all(&tmp).ok();
  }
}
