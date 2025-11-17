use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("books")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Book::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Book::Title).string().not_null())
                    .col(ColumnDef::new(Book::Author).string().not_null())
                    .col(ColumnDef::new(Book::Isbn).string().unique_key())
                    .col(ColumnDef::new(Book::TotalCopies).integer().not_null())
                    .col(ColumnDef::new(Book::AvailableCopies).integer().not_null())
                    .col(ColumnDef::new(Book::PublishedYear).integer().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Book {
    Table,
    Id,
    Title,
    Author,
    Isbn,
    TotalCopies,
    AvailableCopies,
    PublishedYear,
}
