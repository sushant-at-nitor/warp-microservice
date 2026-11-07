use async_trait::async_trait;
use common::{AppError, Paged};
use domain::{aggregate::book::Book, queries::BookQuery};

#[async_trait]
pub trait BookService: Send + Sync {
  async fn get_all(&self) -> Result<Vec<Book>, AppError>;
  async fn list(&self, page: u64, page_size: u64) -> Result<Paged<Book>, AppError>;
  async fn get_by_id(&self, id: i32) -> Result<Option<Book>, AppError>;
  async fn get_available(&self) -> Result<Vec<Book>, AppError>;
  async fn search(&self, query: BookQuery) -> Result<Vec<Book>, AppError>;
  async fn create(&self, book: Book) -> Result<Book, AppError>;
  async fn update(&self, book: Book) -> Result<Book, AppError>;
  async fn delete(&self, id: i32) -> Result<Option<i32>, AppError>;
  async fn borrow_book(&self, book_id: i32, member_id: i32) -> Result<Book, AppError>;
  async fn return_book(&self, book_id: i32, member_id: i32) -> Result<Book, AppError>;
}
