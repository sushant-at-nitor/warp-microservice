use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table("members")
          .if_not_exists()
          .col(
            ColumnDef::new(Member::Id)
              .integer()
              .not_null()
              .primary_key()
              .auto_increment(),
          )
          .col(ColumnDef::new(Member::Name).string().not_null())
          .col(ColumnDef::new(Member::Email).string().not_null().unique_key())
          .col(ColumnDef::new(Member::Age).integer())
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Member::Table).to_owned())
      .await
  }
}

#[derive(Iden)]
enum Member {
  Table,
  Id,
  Name,
  Email,
  Age,
}
