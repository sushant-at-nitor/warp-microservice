use std::sync::Arc;

use application::books::{
  BookService, BorrowBookCommandHandler, BorrowBookCommandHandlerTrait,
  CreateBookCommandHandler, CreateBookCommandHandlerTrait,
  DeleteBookCommandHandler, DeleteBookCommandHandlerTrait,
  GetAllBooksQueryHandler, GetAllBooksQueryHandlerTrait,
  GetAvailableBooksQueryHandler, GetAvailableBooksQueryHandlerTrait,
  GetBookByIdQueryHandler, GetBookByIdQueryHandlerTrait,
  ListBooksQueryHandler, ListBooksQueryHandlerTrait,
  ReturnBookCommandHandler, ReturnBookCommandHandlerTrait,
  SearchBooksQueryHandler, SearchBooksQueryHandlerTrait,
  UpdateBookCommandHandler, UpdateBookCommandHandlerTrait,
};
use common::{AppError, Paged};
use domain::{
  aggregate::book::Book,
  commands::{
    BorrowBookCommand, CreateBookCommand, DeleteBookCommand,
    ReturnBookCommand, UpdateBookCommand,
  },
  queries::{BookQuery, GetAllBooksQuery, GetBookByIdQuery, ListBooks},
};
use sea_orm::DbErr;

// ── Shared mock ──────────────────────────────────────────────────────────────

struct MockBookService {
  book: Book,
  get_by_id_returns_none: bool,
  borrow_fails: bool,
}

impl MockBookService {
  fn ok(book: Book) -> Self {
    Self { book, get_by_id_returns_none: false, borrow_fails: false }
  }
  fn not_found(book: Book) -> Self {
    Self { book, get_by_id_returns_none: true, borrow_fails: false }
  }
  fn no_copies(book: Book) -> Self {
    Self { book, get_by_id_returns_none: false, borrow_fails: true }
  }
}

#[async_trait::async_trait]
impl BookService for MockBookService {
  async fn get_all(&self) -> Result<Vec<Book>, DbErr> {
    Ok(vec![self.book.clone()])
  }
  async fn list(
    &self,
    page: u64,
    page_size: u64,
  ) -> Result<Paged<Book>, DbErr> {
    Ok(Paged {
      items: vec![self.book.clone()],
      total: 1,
      page,
      page_size,
      total_pages: 1,
    })
  }
  async fn get_by_id(&self, _id: i32) -> Result<Option<Book>, DbErr> {
    if self.get_by_id_returns_none {
      Ok(None)
    } else {
      Ok(Some(self.book.clone()))
    }
  }
  async fn get_available(&self) -> Result<Vec<Book>, DbErr> {
    Ok(vec![self.book.clone()])
  }
  async fn search(&self, _query: BookQuery) -> Result<Vec<Book>, DbErr> {
    Ok(vec![self.book.clone()])
  }
  async fn create(&self, book: Book) -> Result<Book, DbErr> {
    Ok(book)
  }
  async fn update(&self, book: Book) -> Result<Book, DbErr> {
    Ok(book)
  }
  async fn delete(&self, id: i32) -> Result<Option<i32>, DbErr> {
    Ok(Some(id))
  }
  async fn borrow_book(
    &self,
    _book_id: i32,
    _member_id: i32,
  ) -> Result<Book, AppError> {
    if self.get_by_id_returns_none {
      Err(AppError::NotFound("Book not found".into()))
    } else if self.borrow_fails {
      Err(AppError::ValidationError("No copies available".into()))
    } else {
      Ok(self.book.clone())
    }
  }
  async fn return_book(
    &self,
    _book_id: i32,
    _member_id: i32,
  ) -> Result<Book, AppError> {
    if self.get_by_id_returns_none {
      Err(AppError::NotFound("Book not found".into()))
    } else {
      Ok(self.book.clone())
    }
  }
}

fn sample_book() -> Book {
  Book::new(
    1,
    "Clean Architecture".into(),
    "Robert Martin".into(),
    "978-0-13-468599-1".into(),
    5,
    5,
    Some(2017),
  )
}

// ── Command handler tests ─────────────────────────────────────────────────────

#[tokio::test]
async fn create_handler_returns_created_book() {
  let svc = Arc::new(MockBookService::ok(sample_book()));
  let handler = CreateBookCommandHandler::new(svc);
  let cmd = CreateBookCommand {
    title: "New Book".into(),
    author: "Author".into(),
    isbn: "123".into(),
    total_copies: 3,
    published_year: None,
  };
  let result = handler.handle(cmd).await;
  assert!(result.is_ok());
  assert_eq!(result.unwrap().title, "New Book");
}

#[tokio::test]
async fn update_handler_updates_book() {
  let svc = Arc::new(MockBookService::ok(sample_book()));
  let handler = UpdateBookCommandHandler::new(svc);
  let cmd = UpdateBookCommand {
    id: 1,
    title: Some("Updated Title".into()),
    author: None,
    isbn: None,
    total_copies: None,
    published_year: None,
  };
  let result = handler.handle(cmd).await;
  assert!(result.is_ok());
  assert_eq!(result.unwrap().title, "Updated Title");
}

#[tokio::test]
async fn update_handler_returns_error_when_not_found() {
  let svc = Arc::new(MockBookService::not_found(sample_book()));
  let handler = UpdateBookCommandHandler::new(svc);
  let cmd = UpdateBookCommand {
    id: 99,
    title: None,
    author: None,
    isbn: None,
    total_copies: None,
    published_year: None,
  };
  let result = handler.handle(cmd).await;
  assert!(result.is_err());
  assert!(matches!(result.unwrap_err(), AppError::ValidationError(_)));
}

#[tokio::test]
async fn delete_handler_returns_deleted_id() {
  let svc = Arc::new(MockBookService::ok(sample_book()));
  let handler = DeleteBookCommandHandler::new(svc);
  let result = handler.handle(DeleteBookCommand { id: 1 }).await;
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), Some(1));
}

#[tokio::test]
async fn borrow_handler_borrows_successfully() {
  let svc = Arc::new(MockBookService::ok(sample_book()));
  let handler = BorrowBookCommandHandler::new(svc);
  let result =
    handler.handle(BorrowBookCommand { book_id: 1, member_id: 42 }).await;
  assert!(result.is_ok());
}

#[tokio::test]
async fn borrow_handler_propagates_not_found() {
  let svc = Arc::new(MockBookService::not_found(sample_book()));
  let handler = BorrowBookCommandHandler::new(svc);
  let result =
    handler.handle(BorrowBookCommand { book_id: 99, member_id: 42 }).await;
  assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
}

#[tokio::test]
async fn borrow_handler_propagates_validation_when_no_copies() {
  let svc = Arc::new(MockBookService::no_copies(sample_book()));
  let handler = BorrowBookCommandHandler::new(svc);
  let result =
    handler.handle(BorrowBookCommand { book_id: 1, member_id: 42 }).await;
  assert!(matches!(result.unwrap_err(), AppError::ValidationError(_)));
}

#[tokio::test]
async fn return_handler_returns_successfully() {
  let svc = Arc::new(MockBookService::ok(sample_book()));
  let handler = ReturnBookCommandHandler::new(svc);
  let result =
    handler.handle(ReturnBookCommand { book_id: 1, member_id: 42 }).await;
  assert!(result.is_ok());
}

#[tokio::test]
async fn return_handler_propagates_not_found() {
  let svc = Arc::new(MockBookService::not_found(sample_book()));
  let handler = ReturnBookCommandHandler::new(svc);
  let result =
    handler.handle(ReturnBookCommand { book_id: 99, member_id: 42 }).await;
  assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
}

// ── Query handler tests ───────────────────────────────────────────────────────

#[tokio::test]
async fn get_all_handler_returns_all_books() {
  let svc = Arc::new(MockBookService::ok(sample_book()));
  let handler = GetAllBooksQueryHandler::new(svc);
  let result = handler.handle(GetAllBooksQuery).await;
  assert!(result.is_ok());
  assert_eq!(result.unwrap().len(), 1);
}

#[tokio::test]
async fn get_by_id_handler_returns_book() {
  let svc = Arc::new(MockBookService::ok(sample_book()));
  let handler = GetBookByIdQueryHandler::new(svc);
  let result = handler.handle(GetBookByIdQuery { id: 1 }).await;
  assert!(result.is_ok());
  assert!(result.unwrap().is_some());
}

#[tokio::test]
async fn get_by_id_handler_returns_none_when_missing() {
  let svc = Arc::new(MockBookService::not_found(sample_book()));
  let handler = GetBookByIdQueryHandler::new(svc);
  let result = handler.handle(GetBookByIdQuery { id: 99 }).await;
  assert!(result.is_ok());
  assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn list_handler_returns_paged_result() {
  let svc = Arc::new(MockBookService::ok(sample_book()));
  let handler = ListBooksQueryHandler::new(svc);
  let result =
    handler.handle(ListBooks { page: Some(1), page_size: Some(10) }).await;
  assert!(result.is_ok());
  let paged = result.unwrap();
  assert_eq!(paged.total, 1);
  assert_eq!(paged.items.len(), 1);
}

#[tokio::test]
async fn get_available_handler_returns_available_books() {
  let svc = Arc::new(MockBookService::ok(sample_book()));
  let handler = GetAvailableBooksQueryHandler::new(svc);
  let result = handler.handle().await;
  assert!(result.is_ok());
  assert_eq!(result.unwrap().len(), 1);
}

#[tokio::test]
async fn search_handler_returns_matching_books() {
  let svc = Arc::new(MockBookService::ok(sample_book()));
  let handler = SearchBooksQueryHandler::new(svc);
  let query = BookQuery {
    year: None,
    name: Some("Clean".into()),
    author: None,
    isbn: None,
  };
  let result = handler.handle(query).await;
  assert!(result.is_ok());
  assert_eq!(result.unwrap().len(), 1);
}
