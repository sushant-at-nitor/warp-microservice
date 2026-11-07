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
