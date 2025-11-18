use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "books")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub title: String,
  pub author: String,
  pub isbn: String,
  pub total_copies: i32,
  pub available_copies: i32,
  pub published_year: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
