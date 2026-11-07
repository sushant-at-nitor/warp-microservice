use crate::repositories::book_repository::BookRepository;
use application::books::BookService;
use async_trait::async_trait;
use common::{AppError, Paged};
use domain::{aggregate::book::Book, queries::BookQuery};

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
  async fn get_all(&self) -> Result<Vec<Book>, AppError> {
    self
      .repo
      .find_all()
      .await
      .map(|models| models.into_iter().map(Into::into).collect())
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }

  async fn get_by_id(&self, id: i32) -> Result<Option<Book>, AppError> {
    self
      .repo
      .find_by_id(id)
      .await
      .map(|opt| opt.map(Into::into))
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }

  async fn get_available(&self) -> Result<Vec<Book>, AppError> {
    self
      .repo
      .find_available_books()
      .await
      .map(|models| models.into_iter().map(Into::into).collect())
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }

  async fn search(&self, query: BookQuery) -> Result<Vec<Book>, AppError> {
    self
      .repo
      .search_books(query)
      .await
      .map(|models| models.into_iter().map(Into::into).collect())
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }

  async fn list(&self, page: u64, page_size: u64) -> Result<Paged<Book>, AppError> {
    let p = self
      .repo
      .list_books(page, page_size)
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(Paged {
      items: p.items.into_iter().map(Into::into).collect(),
      total: p.total,
      page: p.page,
      page_size: p.page_size,
      total_pages: p.total_pages,
    })
  }

  async fn create(&self, book: Book) -> Result<Book, AppError> {
    self
      .repo
      .insert(book.into())
      .await
      .map(Into::into)
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }

  async fn update(&self, book: Book) -> Result<Book, AppError> {
    self
      .repo
      .update(book.into())
      .await
      .map(Into::into)
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }

  async fn delete(&self, id: i32) -> Result<Option<i32>, AppError> {
    let result = self
      .repo
      .delete(id)
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(if result.rows_affected == 0 { None } else { Some(id) })
  }

  async fn borrow_book(&self, book_id: i32, member_id: i32) -> Result<Book, AppError> {
    let model = self
      .repo
      .find_by_id(book_id)
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?
      .ok_or_else(|| AppError::NotFound(format!("Book {} not found", book_id)))?;

    let mut book: Book = model.into();
    book
      .borrow_copy(member_id)
      .map_err(|msg| AppError::ValidationError(msg.to_string()))?;

    let events = book.take_events();
    let result = self
      .repo
      .update(book.into())
      .await
      .map(Into::into)
      .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    for event in events {
      log::info!("[domain-event] {:?}", event);
    }

    Ok(result)
  }

  async fn return_book(&self, book_id: i32, member_id: i32) -> Result<Book, AppError> {
    let model = self
      .repo
      .find_by_id(book_id)
      .await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?
      .ok_or_else(|| AppError::NotFound(format!("Book {} not found", book_id)))?;

    let mut book: Book = model.into();
    book.return_copy(member_id);

    let events = book.take_events();
    let result = self
      .repo
      .update(book.into())
      .await
      .map(Into::into)
      .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    for event in events {
      log::info!("[domain-event] {:?}", event);
    }

    Ok(result)
  }
}
