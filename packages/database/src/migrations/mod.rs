use sea_orm_migration::prelude::*;

mod book;
mod m20261107172120_create_member_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(book::Migration),
      Box::new(m20261107172120_create_member_table::Migration),
    ]
  }
}
