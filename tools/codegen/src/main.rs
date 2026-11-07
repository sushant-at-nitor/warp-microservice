mod api_generator;
mod app_generator;
mod domain_generator;
mod field_def;
mod helpers;
mod infra_generator;

use std::path::Path;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: codegen <EntityName>");
    eprintln!("Example: cargo run -p codegen -- Member");
    std::process::exit(1);
  }

  let entity = &args[1];
  let output_dir = Path::new(".");

  println!("Generating scaffold for: {}", entity);

  domain_generator::generate(entity, output_dir);
  app_generator::generate(entity, output_dir);
  infra_generator::generate(entity, output_dir);
  api_generator::generate(entity, output_dir);

  println!(
    "\nDone! Add the generated files to their respective mod.rs files."
  );
}
