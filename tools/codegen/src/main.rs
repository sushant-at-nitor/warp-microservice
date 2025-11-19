use clap::Parser;

mod generators;
mod writers;
mod sql_parser;

#[derive(Parser, Debug)]
struct Args {
  /// SQL CREATE TABLE Statement
  #[arg(long)]
  sql: String,

  /// Name of the domain aggregate to generate
  #[arg(long)]
  name: String,
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let schema = sql_parser::parse_create_table(&args.sql)?;

  println!("Parsed table schema: {:?}", schema);

  let expanded =
    generators::domain_generator::generate(&args.name, schema.clone())?;

  println!("{expanded}");

  let expanded = generators::commands_generator::generate(schema.clone())?;

  println!("{expanded}");

  let expanded = generators::queries_generator::generate(schema.clone())?;

  println!("{expanded}");

  let expanded = generators::migration_generator::generate(schema.clone())?;

  println!("{expanded}");

  Ok(())
}
