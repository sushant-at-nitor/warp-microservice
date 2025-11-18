use async_trait::async_trait;
use common::Paged;
use domain::{aggregate::book::Book, queries::BookQuery};
use sea_orm::DbErr;

#[async_trait]
pub trait BookService: Send + Sync {
  async fn get_all(&self) -> Result<Vec<Book>, DbErr>;
  async fn list(&self, page: u64, page_size: u64)
  -> Result<Paged<Book>, DbErr>;
  async fn get_by_id(&self, id: i32) -> Result<Option<Book>, DbErr>;
  async fn get_available(&self) -> Result<Vec<Book>, DbErr>;
  async fn search(&self, query: BookQuery) -> Result<Vec<Book>, DbErr>;
  async fn create(&self, book: Book) -> Result<Book, DbErr>;
  async fn update(&self, book: Book) -> Result<Book, DbErr>;
  async fn delete(&self, id: i32) -> Result<Option<i32>, DbErr>;
}
