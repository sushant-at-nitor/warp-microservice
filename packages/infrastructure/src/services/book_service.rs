use crate::repositories::book_repository::BookRepository;
use application::books::BookService;
use async_trait::async_trait;
use common::{AppError, Paged};
use domain::{aggregate::book::Book, queries::BookQuery};
use sea_orm::DbErr;

pub struct BookServiceImpl {
  repo: BookRepository,
}

impl BookServiceImpl {
  pub fn new(repo: BookRepository) -> Self {
    Self { repo }
  }
}

#[async_trait]
impl BookService for BookServiceImpl {
  async fn get_all(&self) -> Result<Vec<Book>, DbErr> {
    let models = self.repo.find_all().await?;
    Ok(models.into_iter().map(Into::into).collect())
  }

  async fn get_by_id(&self, id: i32) -> Result<Option<Book>, DbErr> {
    Ok(self.repo.find_by_id(id).await?.map(Into::into))
  }

  async fn get_available(&self) -> Result<Vec<Book>, DbErr> {
    let models = self.repo.find_available_books().await?;
    Ok(models.into_iter().map(Into::into).collect())
  }

  async fn search(&self, query: BookQuery) -> Result<Vec<Book>, DbErr> {
    let models = self.repo.search_books(query).await?;
    Ok(models.into_iter().map(Into::into).collect())
  }

  async fn list(
    &self,
    page: u64,
    page_size: u64,
  ) -> Result<Paged<Book>, DbErr> {
    let page = self.repo.list_books(page, page_size).await?;
    Ok(Paged {
      items: page.items.into_iter().map(Into::into).collect(),
      total: page.total,
      page: page.page,
      page_size: page.page_size,
      total_pages: page.total_pages,
    })
  }

  async fn create(&self, book: Book) -> Result<Book, DbErr> {
    let active_model = book.into();
    let inserted = self.repo.insert(active_model).await?;
    Ok(inserted.into())
  }

  async fn update(&self, book: Book) -> Result<Book, DbErr> {
    let active_model = book.into();
    let updated = self.repo.update(active_model).await?;
    Ok(updated.into())
  }

  async fn delete(&self, id: i32) -> Result<Option<i32>, DbErr> {
    self.repo.delete(id).await?;
    Ok(Some(id))
  }

  async fn borrow_book(
    &self,
    book_id: i32,
    member_id: i32,
  ) -> Result<Book, AppError> {
    let model = self
      .repo
      .find_by_id(book_id)
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?
      .ok_or_else(|| {
        AppError::NotFound(format!("Book {} not found", book_id))
      })?;

    let mut book: Book = model.into();

    book
      .borrow_copy(member_id)
      .map_err(|msg| AppError::ValidationError(msg.to_string()))?;

    self
      .repo
      .update(book.into())
      .await
      .map(|m| m.into())
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }

  async fn return_book(
    &self,
    book_id: i32,
    member_id: i32,
  ) -> Result<Book, AppError> {
    let model = self
      .repo
      .find_by_id(book_id)
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?
      .ok_or_else(|| {
        AppError::NotFound(format!("Book {} not found", book_id))
      })?;

    let mut book: Book = model.into();
    book.return_copy(member_id);

    self
      .repo
      .update(book.into())
      .await
      .map(|m| m.into())
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }
}
