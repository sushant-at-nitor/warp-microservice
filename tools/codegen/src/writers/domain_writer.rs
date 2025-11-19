use crate::generators::{
  generate_aggregate, generate_commands, generate_migration, generate_queries,
};
use crate::sql_parser::TableSchema;
use crate::writers::write_mod;
use crate::writers::writer::write_file;

pub fn write(
  project_root: &str,
  name: &str,
  schema: TableSchema,
) -> anyhow::Result<()> {
  let pascal = schema.name.pascal.clone();
  let snake = schema.name.snake.clone();

  //
  // ---- Domain Layer ----
  //
  let aggregate_dir = format!("{}/domain/src/aggregate", project_root);
  let aggregate_path = format!("{}/{}.rs", aggregate_dir, snake);
  write_file(&aggregate_path, &generate_aggregate(name, schema.clone())?)?;
  write_mod(&aggregate_dir, &snake)?;

  let commands_dir = format!("{}/domain/src/commands", project_root);
  let commands_path = format!("{}/{}_commands.rs", commands_dir, snake);
  write_file(&commands_path, &generate_commands(schema.clone())?)?;
  write_mod(&commands_dir, &format!("{}_commands", snake))?;

  //   let events_path =
  //     format!("{}/domain/src/events/{}_events.rs", project_root, snake);
  //   write_file(&events_path, &generate_events(schema)?)?;

  let queries_dir = format!("{}/domain/src/queries", project_root);
  let queries_path = format!("{}/{}_queries.rs", queries_dir, snake);
  write_file(&queries_path, &generate_queries(schema.clone())?)?;
  write_mod(&queries_dir, &format!("{}_queries", snake))?;

  //
  // ---- Migration ----
  //
  let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
  let mig_dir = format!("{}/database/src/migrations", project_root);
  let mig_filename = format!("{}_create_{}.rs", timestamp, snake);
  let mig_path = format!("{}/{}", mig_dir, mig_filename);
  write_file(&mig_path, &generate_migration(schema.clone())?)?;

  // migration uses plain filename without extension
  let mig_mod_name = mig_filename.trim_end_matches(".rs");
  write_mod(&mig_dir, mig_mod_name)?;

  Ok(())
}
