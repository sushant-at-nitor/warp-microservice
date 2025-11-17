use crate::entities::book;
use common::Paged;
use domain::queries::BookQuery;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr,
  DeleteResult, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};

pub struct BookRepository {
  db: DatabaseConnection,
}

impl BookRepository {
  pub fn new(db: DatabaseConnection) -> Self {
    Self { db }
  }

  pub async fn find_all(&self) -> Result<Vec<book::Model>, DbErr> {
    book::Entity::find().all(&self.db).await
  }

  pub async fn list_books(
    &self,
    page: u64,
    page_size: u64,
  ) -> Result<Paged<book::Model>, DbErr> {
    let page = page.max(1);
    let page_size = page_size.clamp(1, 100);
    let offset = (page - 1) * page_size;

    let total = book::Entity::find().count(&self.db).await?;

    let items = book::Entity::find()
      .offset(offset)
      .limit(page_size)
      .all(&self.db)
      .await?;

    let total_pages = (total + page_size - 1) / page_size;

    Ok(Paged {
      items,
      total,
      page,
      page_size,
      total_pages,
    })
  }

  pub async fn find_by_id(
    &self,
    id: i32,
  ) -> Result<Option<book::Model>, DbErr> {
    book::Entity::find_by_id(id).one(&self.db).await
  }

  pub async fn find_available_books(&self) -> Result<Vec<book::Model>, DbErr> {
    let result = book::Entity::find()
      .filter(book::Column::AvailableCopies.gt(0))
      .all(&self.db)
      .await?;

    Ok(result)
  }

  pub async fn search_books(
    &self,
    query: BookQuery,
  ) -> Result<Vec<book::Model>, DbErr> {
    use book::Column;

    let condition = Condition::all()
      .add_option(query.year.map(|v| Column::PublishedYear.eq(v)))
      .add_option(query.name.map(|v| Column::Title.contains(v)))
      .add_option(query.author.map(|v| Column::Author.contains(v)))
      .add_option(query.isbn.map(|v| Column::Isbn.eq(v)));

    // if let Some(year) = query.year {
    //   condition = condition.add(Column::PublishedYear.eq(year));
    // }
    // if let Some(name) = query.name {
    //   condition = condition.add(Column::Title.contains(name));
    // }
    // if let Some(author) = query.author {
    //   condition = condition.add(Column::Author.contains(author));
    // }
    // if let Some(isbn) = query.isbn {
    //   condition = condition.add(Column::Isbn.eq(isbn));
    // }

    let result = book::Entity::find().filter(condition).all(&self.db).await?;

    Ok(result)
  }

  pub async fn insert(
    &self,
    model: book::ActiveModel,
  ) -> Result<book::Model, DbErr> {
    model.insert(&self.db).await
  }

  pub async fn update(
    &self,
    model: book::ActiveModel,
  ) -> Result<book::Model, DbErr> {
    model.update(&self.db).await
  }

  pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
    book::Entity::delete_by_id(id).exec(&self.db).await
  }
}
