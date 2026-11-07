# warp-microservice Full Improvement Pipeline — Implementation Plan

> **For agentic workers:** Use `executing-plans` skill to implement task by task.
> Steps use `- [ ]` checkbox syntax for tracking.

**Goal:** Fix all known bugs, complete missing features, add structured error handling, implement the borrow/return domain flow, write a full test suite, and implement the codegen scaffold tool.

**Spec:** `docs/specs/2026-11-07-warp-microservice-improvement-spec.md`

**Architecture:** Clean architecture (domain → application → infrastructure → api) is preserved throughout. Error handling is centralised via an `ApiError` wrapper in the `api` crate — `common` stays framework-free. New features (borrow/return, endpoints) follow the existing CQRS handler pattern exactly. The codegen tool generates full 5-layer scaffolding from a domain name.

**Tech Stack:** Rust 2024 edition, Warp 0.4.2, SeaORM 1.1.17, SQLite (dev), Tokio, serde_json

---

## Stage 1 — Bug Fixes

---

```yaml
task:
  id: 1
  description: "Fix mapper: available_copies mapped from wrong field"
  skill: null
  parallel: false
  depends_on: []
  steps:
    - "Open packages/infrastructure/src/mappers/book.rs"
    - "Change line 12: available_copies: model.total_copies → model.available_copies"
    - "Run cargo build -p infrastructure"
  expected_output: "infrastructure compiles without error"
  verification_cmd: "cargo build -p infrastructure"
  blockers: []
```

**T1 steps:**
- [ ] Edit `packages/infrastructure/src/mappers/book.rs` line 12:
  ```rust
  // Before
  available_copies: model.total_copies,
  // After
  available_copies: model.available_copies,
  ```
- [ ] Run `cargo build -p infrastructure` — confirm GREEN

---

```yaml
task:
  id: 2
  description: "Rename handle_create_book → handle_delete_book in delete.rs"
  skill: null
  parallel: true
  depends_on: []
  steps:
    - "Open packages/api/src/endpoints/books/delete.rs"
    - "Rename async fn handle_create_book → handle_delete_book (line 37 and line 20)"
    - "Run cargo build -p api"
  expected_output: "api compiles without error"
  verification_cmd: "cargo build -p api"
  blockers: []
```

**T2 steps:**
- [ ] Edit `packages/api/src/endpoints/books/delete.rs`:
  - Line 20: `.and_then(handle_create_book)` → `.and_then(handle_delete_book)`
  - Line 37: `async fn handle_create_book` → `async fn handle_delete_book`
- [ ] Run `cargo build -p api` — confirm GREEN

---

```yaml
task:
  id: 3
  description: "Fix unused err variable in get_all.rs (compiler warning)"
  skill: null
  parallel: true
  depends_on: []
  steps:
    - "Open packages/api/src/endpoints/books/get_all.rs line 47"
    - "Change Err(err) => to Err(_) =>"
    - "Run cargo build -p api"
  expected_output: "No unused variable warning in api"
  verification_cmd: "cargo build -p api 2>&1 | grep -v warning"
  blockers: []
```

**T3 steps:**
- [ ] Edit `packages/api/src/endpoints/books/get_all.rs` line 47:
  ```rust
  // Before
  Err(err) => Err(warp::reject::custom(ServerError)),
  // After
  Err(_) => Err(warp::reject::custom(ServerError)),
  ```
- [ ] Run `cargo build -p api` — confirm GREEN

---

```yaml
task:
  id: 4
  description: "Remove DB credential leak and fix panic in database.rs"
  skill: null
  parallel: true
  depends_on: []
  steps:
    - "Open packages/database/src/database.rs"
    - "Remove line 15: println! that leaks DATABASE_URL and panics on missing var"
    - "Fix line 17: db_url is now used safely via unwrap_or_else fallback"
    - "Run cargo build -p database"
  expected_output: "database compiles; no println! referencing DATABASE_URL"
  verification_cmd: "cargo build -p database"
  blockers: []
```

**T4 steps:**
- [ ] Edit `packages/database/src/database.rs` — replace the `connect` function body:
  ```rust
  pub async fn connect() -> Result<Self, DbErr> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
      .unwrap_or_else(|_| "sqlite://book_lending.db?mode=rwc".to_string());

    log::debug!("Connecting to database");
    let connection = SeaDatabase::connect(&db_url).await?;

    Ok(Self { connection })
  }
  ```
- [ ] Add `use log;` import if not present (it's a workspace dep already available)
- [ ] Run `cargo build -p database` — confirm GREEN

---

```yaml
task:
  id: 5
  description: "Fix bind address: 127.0.0.1 → 0.0.0.0 in main.rs"
  skill: null
  parallel: true
  depends_on: []
  steps:
    - "Open packages/api/src/main.rs line 28"
    - "Change [127, 0, 0, 1] to [0, 0, 0, 0]"
    - "Run cargo build -p api"
  expected_output: "api compiles"
  verification_cmd: "cargo build -p api"
  blockers: []
```

**T5 steps:**
- [ ] Edit `packages/api/src/main.rs` line 28:
  ```rust
  // Before
  .run(([127, 0, 0, 1], port.parse().unwrap_or(3030)))
  // After
  .run(([0, 0, 0, 0], port.parse().unwrap_or(3030)))
  ```
- [ ] Run `cargo build -p api` — confirm GREEN

---

```yaml
task:
  id: 6
  description: "Fix Dockerfile COPY paths that escape build context"
  skill: null
  parallel: true
  depends_on: []
  steps:
    - "Open packages/api/Dockerfile"
    - "Replace COPY ../../Cargo.toml ../../Cargo.lock ./ with paths relative to build context root"
    - "Verify docker-compose.yml build context if present"
  expected_output: "Dockerfile COPY paths are within the build context"
  verification_cmd: "grep -n 'COPY' packages/api/Dockerfile"
  blockers: []
```

**T6 steps:**
- [ ] Read `packages/api/Dockerfile` to see the full contents
- [ ] The build context in docker-compose is likely `./` (workspace root) or `./packages/api`
- [ ] If context is workspace root, correct COPY to:
  ```dockerfile
  COPY Cargo.toml Cargo.lock ./
  COPY packages/ packages/
  ```
- [ ] Run `grep -n 'COPY' packages/api/Dockerfile` — confirm paths make sense

---

```yaml
task:
  id: 7
  description: "Remove sea-orm-cli runtime dependency from api/Cargo.toml + clean dead code"
  skill: null
  parallel: true
  depends_on: []
  steps:
    - "Remove sea-orm-cli from packages/api/Cargo.toml [dependencies]"
    - "Remove all commented-out code blocks from update_handler.rs and book_repository.rs"
    - "Run cargo build"
  expected_output: "Full workspace builds cleanly"
  verification_cmd: "cargo build"
  blockers: []
```

**T7 steps:**
- [ ] Edit `packages/api/Cargo.toml` — remove line `sea-orm-cli = { version = "1.1.0" }`
- [ ] Edit `packages/application/src/books/command_handlers/update_handler.rs` — remove commented blocks (lines 29-31 and 53-68)
- [ ] Edit `packages/infrastructure/src/repositories/book_repository.rs` — remove commented blocks in `search_books` (lines 78-90)
- [ ] Run `cargo build` — confirm full workspace GREEN

---

## Stage 2 — Error Handling Architecture + Missing Endpoints

> Tasks 8–13 must run sequentially: AppError → ApiError → rejection handler → update endpoints → new endpoints.

---

```yaml
task:
  id: 8
  description: "Add NotFound variant to AppError in common crate"
  skill: null
  parallel: false
  depends_on: [7]
  steps:
    - "Write failing test: AppError::NotFound should exist and format correctly"
    - "Run test — RED"
    - "Add NotFound(String) variant to AppError enum"
    - "Run test — GREEN"
  expected_output: "AppError has three variants: DatabaseError, ValidationError, NotFound"
  verification_cmd: "cargo test -p common"
  blockers: []
```

**T8 steps:**
- [ ] Edit `packages/common/src/app_error.rs` — add test first:
  ```rust
  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn app_error_not_found_exists() {
      let e = AppError::NotFound("Book 42 not found".into());
      match e {
        AppError::NotFound(msg) => assert_eq!(msg, "Book 42 not found"),
        _ => panic!("wrong variant"),
      }
    }
  }
  ```
- [ ] Run `cargo test -p common` — confirm RED (variant doesn't exist)
- [ ] Add `NotFound(String)` to the enum:
  ```rust
  #[derive(Debug)]
  pub enum AppError {
    DatabaseError(String),
    ValidationError(String),
    NotFound(String),
  }
  ```
- [ ] Run `cargo test -p common` — confirm GREEN
- [ ] Run `cargo build` — fix any exhaustiveness errors in existing match arms (`infrastructure/services/book_service.rs` doesn't match on `AppError` directly, so no changes needed there; application handlers use `AppError` as opaque return)
- [ ] Commit: `git add -A && git commit -m "Add NotFound variant to AppError"`

---

```yaml
task:
  id: 9
  description: "Create api/src/errors.rs with ApiError type and From<AppError>"
  skill: test-driven-development
  parallel: false
  depends_on: [8]
  steps:
    - "Write failing test: From<AppError> produces correct ApiError variant"
    - "Run test — RED"
    - "Create packages/api/src/errors.rs with ApiError enum + impl warp::reject::Reject + From<AppError>"
    - "Add pub mod errors to packages/api/src/main.rs"
    - "Run test — GREEN"
  expected_output: "api compiles with errors module; From<AppError> conversions are correct"
  verification_cmd: "cargo test -p api"
  blockers: []
```

**T9 steps:**
- [ ] Create `packages/api/src/errors.rs`:
  ```rust
  use common::AppError;
  use serde::Serialize;

  #[derive(Debug, Serialize)]
  pub enum ApiError {
    NotFound(String),
    Validation(String),
    Internal(String),
  }

  impl warp::reject::Reject for ApiError {}

  impl From<AppError> for ApiError {
    fn from(e: AppError) -> Self {
      match e {
        AppError::NotFound(msg) => ApiError::NotFound(msg),
        AppError::ValidationError(msg) => ApiError::Validation(msg),
        AppError::DatabaseError(msg) => ApiError::Internal(msg),
      }
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn not_found_maps_correctly() {
      let api_err = ApiError::from(AppError::NotFound("Book not found".into()));
      assert!(matches!(api_err, ApiError::NotFound(_)));
    }

    #[test]
    fn validation_maps_correctly() {
      let api_err = ApiError::from(AppError::ValidationError("Invalid".into()));
      assert!(matches!(api_err, ApiError::Validation(_)));
    }

    #[test]
    fn database_maps_to_internal() {
      let api_err = ApiError::from(AppError::DatabaseError("DB error".into()));
      assert!(matches!(api_err, ApiError::Internal(_)));
    }
  }
  ```
- [ ] Add `pub mod errors;` to `packages/api/src/main.rs`
- [ ] Run `cargo test -p api` — confirm GREEN
- [ ] Commit: `git add -A && git commit -m "Add ApiError with From<AppError> in api crate"`

---

```yaml
task:
  id: 10
  description: "Create rejection handler and wire recover() in startup.rs"
  skill: null
  parallel: false
  depends_on: [9]
  steps:
    - "Create packages/api/src/rejection.rs with handle_rejection async fn"
    - "Add pub mod rejection to main.rs"
    - "Update startup.rs build_app_state signature — not needed here"
    - "Update endpoints/mod.rs register() return type to add recover"
    - "Run cargo build -p api"
  expected_output: "api compiles; routes have .recover(handle_rejection) wired"
  verification_cmd: "cargo build -p api"
  blockers: []
```

**T10 steps:**
- [ ] Create `packages/api/src/rejection.rs`:
  ```rust
  use serde_json::json;
  use warp::{Rejection, Reply, http::StatusCode};

  use crate::errors::ApiError;

  pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let (status, message) = if err.is_not_found() {
      (StatusCode::NOT_FOUND, "Not found".to_string())
    } else if let Some(e) = err.find::<ApiError>() {
      match e {
        ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
        ApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
        ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
      }
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
      (StatusCode::BAD_REQUEST, format!("Invalid body: {}", e))
    } else {
      (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
    };

    let body = warp::reply::json(&json!({ "error": message, "status": status.as_u16() }));
    Ok(warp::reply::with_status(body, status))
  }
  ```
- [ ] Add `pub mod rejection;` to `packages/api/src/main.rs`
- [ ] Edit `packages/api/src/endpoints/mod.rs` — wrap routes with recover:
  ```rust
  pub mod books;

  use crate::{rejection::handle_rejection, startup::AppState};
  use std::sync::Arc;
  use warp::Filter;

  pub fn register(
    state: Arc<AppState>,
  ) -> impl Filter<Extract = impl warp::Reply> + Clone {
    books::register(state).recover(handle_rejection)
  }
  ```
- [ ] Run `cargo build -p api` — confirm GREEN
- [ ] Commit: `git add -A && git commit -m "Add JSON rejection handler with recover() filter"`

---

```yaml
task:
  id: 11
  description: "Update all 6 existing endpoints to use ApiError instead of ServerError"
  skill: null
  parallel: false
  depends_on: [10]
  steps:
    - "Update create.rs, update.rs, delete.rs, get_all.rs, get_by_id.rs, list.rs"
    - "Replace ServerError usage with ApiError::from(e)"
    - "Remove common::server_error import from each"
    - "Run cargo build -p api"
  expected_output: "api compiles; no imports of ServerError remain in endpoints"
  verification_cmd: "cargo build -p api && grep -rn 'ServerError' packages/api/src/endpoints/"
  blockers: []
```

**T11 steps — apply to all 6 endpoint files:**

For each file, replace the error handler pattern:
```rust
// Before (in every file)
use common::server_error::ServerError;
...
Err(_) => Err(warp::reject::custom(ServerError)),

// After
use crate::errors::ApiError;
...
Err(e) => Err(warp::reject::custom(ApiError::from(e))),
```

- [ ] Edit `packages/api/src/endpoints/books/create.rs`:
  - Remove `use common::server_error::ServerError;`
  - Add `use crate::errors::ApiError;`
  - Change `Err(_) => Err(warp::reject::custom(ServerError))` → `Err(e) => Err(warp::reject::custom(ApiError::from(e)))`

- [ ] Edit `packages/api/src/endpoints/books/update.rs`:
  - Same swap. The existing `Err(_)` becomes `Err(e) => Err(warp::reject::custom(ApiError::from(e)))`

- [ ] Edit `packages/api/src/endpoints/books/delete.rs`:
  - Same swap.

- [ ] Edit `packages/api/src/endpoints/books/get_all.rs`:
  - Same swap. The `Err(_)` (already fixed in T3) → `Err(e) => Err(warp::reject::custom(ApiError::from(e)))`

- [ ] Edit `packages/api/src/endpoints/books/get_by_id.rs`:
  - Same swap.

- [ ] Edit `packages/api/src/endpoints/books/list.rs`:
  - Same swap.

- [ ] Run `cargo build -p api` — confirm GREEN
- [ ] Run `grep -rn 'ServerError' packages/api/src/endpoints/` — confirm zero results
- [ ] Remove `packages/common/src/server_error.rs` and its re-export from `common/src/lib.rs` if no other crate uses it (`grep -rn 'server_error\|ServerError' packages/`)
- [ ] Commit: `git add -A && git commit -m "Replace ServerError with ApiError in all endpoints"`

---

```yaml
task:
  id: 12
  description: "Add GetAvailableBooksQueryHandler and wire GET /books/available"
  skill: test-driven-development
  parallel: false
  depends_on: [11]
  steps:
    - "Create application/src/books/query_handlers/get_available_handler.rs"
    - "Export from query_handlers/mod.rs"
    - "Create api/src/endpoints/books/get_available.rs"
    - "Register route in books/mod.rs"
    - "Run cargo build -p api"
  expected_output: "api compiles; GET /books/available route exists"
  verification_cmd: "cargo build -p api"
  blockers: []
```

**T12 steps:**
- [ ] Create `packages/application/src/books/query_handlers/get_available_handler.rs`:
  ```rust
  use std::sync::Arc;
  use async_trait::async_trait;
  use common::AppError;
  use domain::aggregate::book::Book;
  use crate::books::BookService;

  #[async_trait]
  pub trait GetAvailableBooksQueryHandlerTrait: Send + Sync {
    async fn handle(&self) -> Result<Vec<Book>, AppError>;
  }

  pub struct GetAvailableBooksQueryHandler<T: BookService> {
    service: Arc<T>,
  }

  impl<T: BookService> GetAvailableBooksQueryHandler<T> {
    pub fn new(service: Arc<T>) -> Self {
      Self { service }
    }
  }

  #[async_trait]
  impl<T: BookService> GetAvailableBooksQueryHandlerTrait
    for GetAvailableBooksQueryHandler<T>
  {
    async fn handle(&self) -> Result<Vec<Book>, AppError> {
      self.service.get_available().await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
  }
  ```
- [ ] Edit `packages/application/src/books/query_handlers/mod.rs` — add:
  ```rust
  pub mod get_available_handler;
  pub use get_available_handler::*;
  ```
- [ ] Create `packages/api/src/endpoints/books/get_available.rs`:
  ```rust
  use crate::errors::ApiError;
  use application::books::{GetAvailableBooksQueryHandler, GetAvailableBooksQueryHandlerTrait};
  use application::books::BookService;
  use std::sync::Arc;
  use warp::Filter;

  pub fn route<T>(
    handler: Arc<GetAvailableBooksQueryHandler<T>>,
  ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
  where
    T: BookService + Send + Sync + 'static,
  {
    warp::path!("books" / "available")
      .and(warp::get())
      .and(with_handler(handler))
      .and_then(handle_get_available)
  }

  fn with_handler<T>(
    handler: Arc<GetAvailableBooksQueryHandler<T>>,
  ) -> impl Filter<Extract = (Arc<GetAvailableBooksQueryHandler<T>>,), Error = std::convert::Infallible> + Clone
  where
    T: BookService + Send + Sync + 'static,
  {
    warp::any().map(move || handler.clone())
  }

  async fn handle_get_available<T>(
    handler: Arc<GetAvailableBooksQueryHandler<T>>,
  ) -> Result<impl warp::Reply, warp::Rejection>
  where
    T: BookService + Send + Sync + 'static,
  {
    match handler.handle().await {
      Ok(books) => Ok(warp::reply::json(&books)),
      Err(e) => Err(warp::reject::custom(ApiError::from(e))),
    }
  }
  ```
- [ ] Edit `packages/api/src/endpoints/books/mod.rs` — add:
  - `mod get_available;` to module declarations
  - `Arc::new(GetAvailableBooksQueryHandler::new(service.clone()))` handler instantiation
  - Import `GetAvailableBooksQueryHandler` from application
  - `let get_available = get_available::route(get_available_handler);`
  - Add `.or(get_available)` to the chain
- [ ] Run `cargo build -p api` — confirm GREEN
- [ ] Commit: `git add -A && git commit -m "Add GET /books/available endpoint"`

---

```yaml
task:
  id: 13
  description: "Add SearchBooksQueryHandler and wire GET /books/search"
  skill: test-driven-development
  parallel: false
  depends_on: [12]
  steps:
    - "Create application/src/books/query_handlers/search_handler.rs"
    - "Export from mod.rs"
    - "Create api/src/endpoints/books/search.rs"
    - "Register route in books/mod.rs"
    - "Run cargo build -p api"
  expected_output: "api compiles; GET /books/search?author=... route exists"
  verification_cmd: "cargo build -p api"
  blockers: []
```

**T13 steps:**
- [ ] Create `packages/application/src/books/query_handlers/search_handler.rs`:
  ```rust
  use std::sync::Arc;
  use async_trait::async_trait;
  use common::AppError;
  use domain::{aggregate::book::Book, queries::BookQuery};
  use crate::books::BookService;

  #[async_trait]
  pub trait SearchBooksQueryHandlerTrait: Send + Sync {
    async fn handle(&self, query: BookQuery) -> Result<Vec<Book>, AppError>;
  }

  pub struct SearchBooksQueryHandler<T: BookService> {
    service: Arc<T>,
  }

  impl<T: BookService> SearchBooksQueryHandler<T> {
    pub fn new(service: Arc<T>) -> Self {
      Self { service }
    }
  }

  #[async_trait]
  impl<T: BookService> SearchBooksQueryHandlerTrait for SearchBooksQueryHandler<T> {
    async fn handle(&self, query: BookQuery) -> Result<Vec<Book>, AppError> {
      self.service.search(query).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
  }
  ```
- [ ] Edit `packages/application/src/books/query_handlers/mod.rs` — add:
  ```rust
  pub mod search_handler;
  pub use search_handler::*;
  ```
- [ ] Create `packages/api/src/endpoints/books/search.rs`:
  ```rust
  use crate::errors::ApiError;
  use application::books::{BookService, SearchBooksQueryHandler, SearchBooksQueryHandlerTrait};
  use domain::queries::BookQuery;
  use std::sync::Arc;
  use warp::Filter;

  pub fn route<T>(
    handler: Arc<SearchBooksQueryHandler<T>>,
  ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
  where
    T: BookService + Send + Sync + 'static,
  {
    warp::path!("books" / "search")
      .and(warp::get())
      .and(warp::query::<BookQuery>())
      .and(with_handler(handler))
      .and_then(handle_search)
  }

  fn with_handler<T>(
    handler: Arc<SearchBooksQueryHandler<T>>,
  ) -> impl Filter<Extract = (Arc<SearchBooksQueryHandler<T>>,), Error = std::convert::Infallible> + Clone
  where
    T: BookService + Send + Sync + 'static,
  {
    warp::any().map(move || handler.clone())
  }

  async fn handle_search<T>(
    query: BookQuery,
    handler: Arc<SearchBooksQueryHandler<T>>,
  ) -> Result<impl warp::Reply, warp::Rejection>
  where
    T: BookService + Send + Sync + 'static,
  {
    match handler.handle(query).await {
      Ok(books) => Ok(warp::reply::json(&books)),
      Err(e) => Err(warp::reject::custom(ApiError::from(e))),
    }
  }
  ```
- [ ] Update `packages/api/src/endpoints/books/mod.rs` — register search route (same pattern as T12)
- [ ] Run `cargo build -p api` — confirm GREEN
- [ ] Commit: `git add -A && git commit -m "Add GET /books/search endpoint"`

---

## Stage 3 — BorrowBook / ReturnBook Feature

---

```yaml
task:
  id: 14
  description: "Add BorrowBookCommand and ReturnBookCommand to domain"
  skill: null
  parallel: false
  depends_on: [13]
  steps:
    - "Append two new command structs to packages/domain/src/commands/book.rs"
    - "Run cargo build -p domain"
  expected_output: "domain compiles with two new command types"
  verification_cmd: "cargo build -p domain"
  blockers: []
```

**T14 steps:**
- [ ] Edit `packages/domain/src/commands/book.rs` — append at end:
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct BorrowBookCommand {
    pub book_id: i32,
    pub member_id: i32,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct ReturnBookCommand {
    pub book_id: i32,
    pub member_id: i32,
  }
  ```
- [ ] Run `cargo build -p domain` — confirm GREEN
- [ ] Commit: `git add -A && git commit -m "Add BorrowBookCommand and ReturnBookCommand to domain"`

---

```yaml
task:
  id: 15
  description: "Add borrow_book / return_book to BookService trait"
  skill: null
  parallel: false
  depends_on: [14]
  steps:
    - "Add two method signatures to application/src/books/service.rs"
    - "Run cargo build — expect failures in infrastructure (unimplemented trait methods)"
  expected_output: "trait compiles; infrastructure fails with unimplemented methods"
  verification_cmd: "cargo build -p application"
  blockers: []
```

**T15 steps:**
- [ ] Edit `packages/application/src/books/service.rs`:
  - Add `use common::AppError;` import
  - Add two new method signatures to the trait:
    ```rust
    async fn borrow_book(&self, book_id: i32, member_id: i32) -> Result<Book, AppError>;
    async fn return_book(&self, book_id: i32, member_id: i32) -> Result<Book, AppError>;
    ```
- [ ] Run `cargo build -p application` — confirm GREEN
- [ ] Run `cargo build -p infrastructure` — expect RED (BookServiceImpl missing two methods)
- [ ] Commit: `git add -A && git commit -m "Add borrow_book/return_book to BookService trait"`

---

```yaml
task:
  id: 16
  description: "Implement borrow_book and return_book in BookServiceImpl"
  skill: test-driven-development
  parallel: false
  depends_on: [15]
  steps:
    - "Implement the two new methods in infrastructure/src/services/book_service.rs"
    - "Run cargo build — confirm full workspace GREEN"
  expected_output: "Full workspace compiles"
  verification_cmd: "cargo build"
  blockers: []
```

**T16 steps:**
- [ ] Edit `packages/infrastructure/src/services/book_service.rs`:
  - Add `use common::AppError;` import
  - Add two method implementations to the `impl BookService for BookServiceImpl` block:
    ```rust
    async fn borrow_book(&self, book_id: i32, member_id: i32) -> Result<Book, AppError> {
      let mut book = self
        .repo
        .find_by_id(book_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .map(Into::into)
        .ok_or_else(|| AppError::NotFound(format!("Book {} not found", book_id)))?;

      book.borrow_copy(member_id)
        .map_err(|msg| AppError::ValidationError(msg.to_string()))?;

      self.repo
        .update(book.into())
        .await
        .map(Into::into)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn return_book(&self, book_id: i32, member_id: i32) -> Result<Book, AppError> {
      let mut book = self
        .repo
        .find_by_id(book_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .map(Into::into)
        .ok_or_else(|| AppError::NotFound(format!("Book {} not found", book_id)))?;

      book.return_copy(member_id);

      self.repo
        .update(book.into())
        .await
        .map(Into::into)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
    ```
- [ ] Run `cargo build` — confirm full workspace GREEN
- [ ] Commit: `git add -A && git commit -m "Implement borrow_book and return_book in BookServiceImpl"`

---

```yaml
task:
  id: 17
  description: "Create BorrowBookCommandHandler"
  skill: test-driven-development
  parallel: false
  depends_on: [16]
  steps:
    - "Create packages/application/src/books/command_handlers/borrow_handler.rs"
    - "Export from command_handlers/mod.rs"
    - "Run cargo build -p application"
  expected_output: "application compiles with BorrowBookCommandHandler"
  verification_cmd: "cargo build -p application"
  blockers: []
```

**T17 steps:**
- [ ] Create `packages/application/src/books/command_handlers/borrow_handler.rs`:
  ```rust
  use std::sync::Arc;
  use async_trait::async_trait;
  use common::AppError;
  use domain::{aggregate::book::Book, commands::BorrowBookCommand};
  use crate::books::BookService;

  #[async_trait]
  pub trait BorrowBookCommandHandlerTrait: Send + Sync {
    async fn handle(&self, cmd: BorrowBookCommand) -> Result<Book, AppError>;
  }

  pub struct BorrowBookCommandHandler<T: BookService> {
    service: Arc<T>,
  }

  impl<T: BookService> BorrowBookCommandHandler<T> {
    pub fn new(service: Arc<T>) -> Self {
      Self { service }
    }
  }

  #[async_trait]
  impl<T: BookService> BorrowBookCommandHandlerTrait for BorrowBookCommandHandler<T> {
    async fn handle(&self, cmd: BorrowBookCommand) -> Result<Book, AppError> {
      self.service.borrow_book(cmd.book_id, cmd.member_id).await
    }
  }
  ```
- [ ] Edit `packages/application/src/books/command_handlers/mod.rs` — add:
  ```rust
  pub mod borrow_handler;
  pub use borrow_handler::*;
  ```
- [ ] Run `cargo build -p application` — confirm GREEN

---

```yaml
task:
  id: 18
  description: "Create ReturnBookCommandHandler"
  skill: test-driven-development
  parallel: true
  depends_on: [16]
  steps:
    - "Create packages/application/src/books/command_handlers/return_handler.rs"
    - "Export from command_handlers/mod.rs"
    - "Run cargo build -p application"
  expected_output: "application compiles with ReturnBookCommandHandler"
  verification_cmd: "cargo build -p application"
  blockers: []
```

**T18 steps:**
- [ ] Create `packages/application/src/books/command_handlers/return_handler.rs`:
  ```rust
  use std::sync::Arc;
  use async_trait::async_trait;
  use common::AppError;
  use domain::{aggregate::book::Book, commands::ReturnBookCommand};
  use crate::books::BookService;

  #[async_trait]
  pub trait ReturnBookCommandHandlerTrait: Send + Sync {
    async fn handle(&self, cmd: ReturnBookCommand) -> Result<Book, AppError>;
  }

  pub struct ReturnBookCommandHandler<T: BookService> {
    service: Arc<T>,
  }

  impl<T: BookService> ReturnBookCommandHandler<T> {
    pub fn new(service: Arc<T>) -> Self {
      Self { service }
    }
  }

  #[async_trait]
  impl<T: BookService> ReturnBookCommandHandlerTrait for ReturnBookCommandHandler<T> {
    async fn handle(&self, cmd: ReturnBookCommand) -> Result<Book, AppError> {
      self.service.return_book(cmd.book_id, cmd.member_id).await
    }
  }
  ```
- [ ] Edit `packages/application/src/books/command_handlers/mod.rs` — add:
  ```rust
  pub mod return_handler;
  pub use return_handler::*;
  ```
- [ ] Run `cargo build -p application` — confirm GREEN
- [ ] Commit (after both T17 + T18): `git add -A && git commit -m "Add BorrowBook and ReturnBook command handlers"`

---

```yaml
task:
  id: 19
  description: "Create borrow/return API routes and register them"
  skill: null
  parallel: false
  depends_on: [17, 18]
  steps:
    - "Create packages/api/src/endpoints/books/borrow.rs"
    - "Create packages/api/src/endpoints/books/return_book.rs"
    - "Register both routes in books/mod.rs"
    - "Run cargo build -p api"
  expected_output: "api compiles; POST /books/:id/borrow and POST /books/:id/return routes exist"
  verification_cmd: "cargo build -p api"
  blockers: []
```

**T19 steps:**
- [ ] Create `packages/api/src/endpoints/books/borrow.rs`:
  ```rust
  use crate::errors::ApiError;
  use application::books::{BookService, BorrowBookCommandHandler, BorrowBookCommandHandlerTrait};
  use domain::commands::BorrowBookCommand;
  use serde::Deserialize;
  use std::sync::Arc;
  use warp::Filter;

  #[derive(Debug, Deserialize)]
  pub struct BorrowBody {
    pub member_id: i32,
  }

  pub fn route<T>(
    handler: Arc<BorrowBookCommandHandler<T>>,
  ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
  where
    T: BookService + Send + Sync + 'static,
  {
    warp::path!("books" / i32 / "borrow")
      .and(warp::post())
      .and(with_handler(handler))
      .and(warp::body::json())
      .and_then(handle_borrow)
  }

  fn with_handler<T>(
    handler: Arc<BorrowBookCommandHandler<T>>,
  ) -> impl Filter<Extract = (Arc<BorrowBookCommandHandler<T>>,), Error = std::convert::Infallible> + Clone
  where
    T: BookService + Send + Sync + 'static,
  {
    warp::any().map(move || handler.clone())
  }

  async fn handle_borrow<T>(
    book_id: i32,
    handler: Arc<BorrowBookCommandHandler<T>>,
    body: BorrowBody,
  ) -> Result<impl warp::Reply, warp::Rejection>
  where
    T: BookService + Send + Sync + 'static,
  {
    let cmd = BorrowBookCommand { book_id, member_id: body.member_id };
    match handler.handle(cmd).await {
      Ok(book) => Ok(warp::reply::json(&book)),
      Err(e) => Err(warp::reject::custom(ApiError::from(e))),
    }
  }
  ```
- [ ] Create `packages/api/src/endpoints/books/return_book.rs` (same pattern, `ReturnBookCommand`, path `"books" / i32 / "return"`):
  ```rust
  use crate::errors::ApiError;
  use application::books::{BookService, ReturnBookCommandHandler, ReturnBookCommandHandlerTrait};
  use domain::commands::ReturnBookCommand;
  use serde::Deserialize;
  use std::sync::Arc;
  use warp::Filter;

  #[derive(Debug, Deserialize)]
  pub struct ReturnBody {
    pub member_id: i32,
  }

  pub fn route<T>(
    handler: Arc<ReturnBookCommandHandler<T>>,
  ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
  where
    T: BookService + Send + Sync + 'static,
  {
    warp::path!("books" / i32 / "return")
      .and(warp::post())
      .and(with_handler(handler))
      .and(warp::body::json())
      .and_then(handle_return)
  }

  fn with_handler<T>(
    handler: Arc<ReturnBookCommandHandler<T>>,
  ) -> impl Filter<Extract = (Arc<ReturnBookCommandHandler<T>>,), Error = std::convert::Infallible> + Clone
  where
    T: BookService + Send + Sync + 'static,
  {
    warp::any().map(move || handler.clone())
  }

  async fn handle_return<T>(
    book_id: i32,
    handler: Arc<ReturnBookCommandHandler<T>>,
    body: ReturnBody,
  ) -> Result<impl warp::Reply, warp::Rejection>
  where
    T: BookService + Send + Sync + 'static,
  {
    let cmd = ReturnBookCommand { book_id, member_id: body.member_id };
    match handler.handle(cmd).await {
      Ok(book) => Ok(warp::reply::json(&book)),
      Err(e) => Err(warp::reject::custom(ApiError::from(e))),
    }
  }
  ```
- [ ] Update `packages/api/src/endpoints/books/mod.rs`:
  - Add `mod borrow; mod return_book;`
  - Instantiate `BorrowBookCommandHandler` and `ReturnBookCommandHandler`
  - Import them from application
  - Add `let borrow = borrow::route(borrow_handler);`
  - Add `let return_book = return_book::route(return_handler);`
  - Add `.or(borrow).or(return_book)` to the filter chain
- [ ] Run `cargo build -p api` — confirm GREEN
- [ ] Commit: `git add -A && git commit -m "Add POST /books/:id/borrow and POST /books/:id/return routes"`

---

## Stage 4 — Test Suite

---

```yaml
task:
  id: 20
  description: "Add warp/test dev-dependencies and create test helper module"
  skill: null
  parallel: false
  depends_on: [19]
  steps:
    - "Add dev-dependencies to packages/api/Cargo.toml: serde_json (already workspace dep)"
    - "Confirm warp test utilities are available (warp has built-in test support)"
    - "Create packages/api/tests/ directory"
    - "Run cargo test -p api — expect 0 tests, compiles cleanly"
  expected_output: "cargo test -p api compiles and runs 0 tests"
  verification_cmd: "cargo test -p api"
  blockers: []
```

**T20 steps:**
- [ ] Check `packages/api/Cargo.toml` for `[dev-dependencies]` section — warp's test utilities are part of `warp` itself (via `warp::test::request()`)
- [ ] Create `packages/api/tests/books_api.rs` with a placeholder:
  ```rust
  // Integration tests for the books API
  ```
- [ ] Run `cargo test -p api` — confirm it compiles (0 tests run is fine here)

---

```yaml
task:
  id: 21
  description: "Write unit tests for command handlers (create, update, delete, borrow, return)"
  skill: test-driven-development
  parallel: false
  depends_on: [20]
  steps:
    - "Add #[cfg(test)] module to each command handler with mock BookService"
    - "Write tests: happy path + error path for each handler"
    - "Run cargo test -p application"
  expected_output: "10 unit tests pass in application crate"
  verification_cmd: "cargo test -p application"
  blockers: []
```

**T21 steps:**

For each command handler, add a `#[cfg(test)]` module. The pattern for a mock is:

```rust
#[cfg(test)]
mod tests {
  use super::*;
  use common::Paged;
  use domain::queries::BookQuery;
  use sea_orm::DbErr;

  struct MockBookService {
    should_fail: bool,
    book: Book,
  }

  #[async_trait::async_trait]
  impl BookService for MockBookService {
    async fn get_all(&self) -> Result<Vec<Book>, DbErr> { Ok(vec![]) }
    async fn list(&self, _: u64, _: u64) -> Result<Paged<Book>, DbErr> {
      Ok(Paged { items: vec![], total: 0, page: 1, page_size: 10, total_pages: 0 })
    }
    async fn get_by_id(&self, _: i32) -> Result<Option<Book>, DbErr> {
      if self.should_fail { Ok(None) } else { Ok(Some(self.book.clone())) }
    }
    async fn get_available(&self) -> Result<Vec<Book>, DbErr> { Ok(vec![]) }
    async fn search(&self, _: BookQuery) -> Result<Vec<Book>, DbErr> { Ok(vec![]) }
    async fn create(&self, book: Book) -> Result<Book, DbErr> { Ok(book) }
    async fn update(&self, book: Book) -> Result<Book, DbErr> {
      if self.should_fail { Err(DbErr::Custom("fail".into())) } else { Ok(book) }
    }
    async fn delete(&self, id: i32) -> Result<Option<i32>, DbErr> { Ok(Some(id)) }
    async fn borrow_book(&self, book_id: i32, _: i32) -> Result<Book, AppError> {
      if self.should_fail { Err(AppError::NotFound("not found".into())) }
      else { Ok(self.book.clone()) }
    }
    async fn return_book(&self, book_id: i32, _: i32) -> Result<Book, AppError> {
      if self.should_fail { Err(AppError::NotFound("not found".into())) }
      else { Ok(self.book.clone()) }
    }
  }

  fn sample_book() -> Book {
    Book::new(1, "Title".into(), "Author".into(), "ISBN".into(), 5, 5, None)
  }
  // ...tests follow
}
```

- [ ] Add `#[cfg(test)]` to `create_handler.rs` — tests: `handle_returns_created_book`, `handle_db_error_returns_database_error`
- [ ] Add `#[cfg(test)]` to `update_handler.rs` — tests: `handle_updates_book`, `handle_not_found_returns_validation_error`
- [ ] Add `#[cfg(test)]` to `delete_handler.rs` — tests: `handle_deletes_book`, `handle_db_error_propagates`
- [ ] Add `#[cfg(test)]` to `borrow_handler.rs` — tests: `handle_borrows_book`, `handle_not_found_propagates`
- [ ] Add `#[cfg(test)]` to `return_handler.rs` — tests: `handle_returns_book`, `handle_not_found_propagates`
- [ ] Run `cargo test -p application` — confirm all pass

---

```yaml
task:
  id: 22
  description: "Write unit tests for query handlers (get_all, get_by_id, list, get_available, search)"
  skill: test-driven-development
  parallel: true
  depends_on: [20]
  steps:
    - "Add #[cfg(test)] module to each query handler"
    - "Run cargo test -p application"
  expected_output: "10 unit tests pass in application query handlers"
  verification_cmd: "cargo test -p application"
  blockers: []
```

**T22 steps:**
- [ ] Add `#[cfg(test)]` to `get_all_handler.rs` — test: `handle_returns_all_books`
- [ ] Add `#[cfg(test)]` to `get_by_id_handler.rs` — tests: `handle_returns_book`, `handle_returns_none_when_not_found`
- [ ] Add `#[cfg(test)]` to `list_handler.rs` — test: `handle_returns_paged_result`
- [ ] Add `#[cfg(test)]` to `get_available_handler.rs` — test: `handle_returns_available_books`
- [ ] Add `#[cfg(test)]` to `search_handler.rs` — test: `handle_returns_matching_books`
- [ ] Run `cargo test -p application` — confirm all pass
- [ ] Commit (after T21 + T22): `git add -A && git commit -m "Add unit tests for all command and query handlers"`

---

```yaml
task:
  id: 23
  description: "Write integration tests hitting real in-memory SQLite via warp::test"
  skill: test-driven-development
  parallel: false
  depends_on: [21, 22]
  steps:
    - "Create full test suite in packages/api/tests/books_api.rs"
    - "Use warp::test::request() — no port needed"
    - "In-memory SQLite: sqlite::memory: via SeaORM"
    - "Run cargo test -p api"
  expected_output: "All integration tests pass"
  verification_cmd: "cargo test -p api -- --test-output immediate"
  blockers: []
```

**T23 steps:**
- [ ] Edit `packages/api/Cargo.toml` — add `[dev-dependencies]` section:
  ```toml
  [dev-dependencies]
  tokio = { workspace = true }
  serde_json = { workspace = true }
  sea-orm = { workspace = true }
  database = { path = "../database" }
  ```
- [ ] Write `packages/api/tests/books_api.rs` with a `setup()` async helper that:
  1. Creates `SeaDatabase::connect("sqlite::memory:").await`
  2. Runs `Migrator::up(&conn, None).await`
  3. Builds `AppState { db: Database { connection: conn } }`
  4. Calls `endpoints::register(Arc::new(state))`
  5. Returns the filter

  Write these test cases using `warp::test::request()`:
  ```rust
  #[tokio::test]
  async fn create_book_returns_200() { ... }

  #[tokio::test]
  async fn get_all_books_returns_200() { ... }

  #[tokio::test]
  async fn get_book_by_id_returns_404_when_missing() { ... }

  #[tokio::test]
  async fn update_book_returns_404_when_missing() { ... }

  #[tokio::test]
  async fn delete_book_returns_404_when_missing() { ... }

  #[tokio::test]
  async fn get_available_books_returns_200() { ... }

  #[tokio::test]
  async fn search_books_returns_200() { ... }

  #[tokio::test]
  async fn borrow_book_returns_422_when_no_copies() { ... }

  #[tokio::test]
  async fn borrow_then_return_cycle_works() { ... }
  ```
- [ ] Run `cargo test -p api` — confirm all pass
- [ ] Commit: `git add -A && git commit -m "Add integration test suite for all API endpoints"`

---

## Stage 5 — Codegen Tool

> Stage 5 is independent of Stages 1–4 and can be executed in parallel after T7 (workspace builds clean).

---

```yaml
task:
  id: 24
  description: "Set up codegen crate entry point and helpers"
  skill: null
  parallel: false
  depends_on: [7]
  steps:
    - "Add tools/codegen to workspace members in root Cargo.toml"
    - "Create tools/codegen/Cargo.toml"
    - "Implement tools/codegen/src/helpers.rs: to_pascal_case, to_snake_case"
    - "Write unit tests for helpers"
    - "Run cargo test -p codegen"
  expected_output: "codegen crate compiles; to_pascal_case and to_snake_case pass tests"
  verification_cmd: "cargo test -p codegen"
  blockers: []
```

**T24 steps:**
- [ ] Edit root `Cargo.toml` — add `"tools/codegen"` to `[workspace] members`
- [ ] Create `tools/codegen/Cargo.toml`:
  ```toml
  [package]
  name = "codegen"
  version = "0.1.0"
  edition = "2024"

  [[bin]]
  name = "codegen"
  path = "src/main.rs"
  ```
- [ ] Create `tools/codegen/src/helpers.rs`:
  ```rust
  pub fn to_pascal_case(s: &str) -> String {
    s.split(['_', '-', ' '])
      .filter(|p| !p.is_empty())
      .map(|p| {
        let mut c = p.chars();
        match c.next() {
          None => String::new(),
          Some(f) => f.to_uppercase().to_string() + c.as_str(),
        }
      })
      .collect()
  }

  pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.char_indices() {
      if ch.is_uppercase() && i > 0 {
        result.push('_');
      }
      result.push(ch.to_lowercase().next().unwrap());
    }
    result
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn pascal_case_from_snake() {
      assert_eq!(to_pascal_case("book_loan"), "BookLoan");
    }

    #[test]
    fn pascal_case_single_word() {
      assert_eq!(to_pascal_case("member"), "Member");
    }

    #[test]
    fn snake_case_from_pascal() {
      assert_eq!(to_snake_case("BookLoan"), "book_loan");
    }

    #[test]
    fn snake_case_single_word() {
      assert_eq!(to_snake_case("Member"), "member");
    }
  }
  ```
- [ ] Create `tools/codegen/src/main.rs` (placeholder):
  ```rust
  mod helpers;
  mod domain_generator;
  mod app_generator;
  mod infra_generator;
  mod api_generator;

  fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
      eprintln!("Usage: codegen <EntityName>");
      std::process::exit(1);
    }
    let entity = &args[1];
    println!("Generating scaffold for: {}", entity);
    // TODO: call generators
  }
  ```
- [ ] Create empty stub files: `domain_generator.rs`, `app_generator.rs`, `infra_generator.rs`, `api_generator.rs` (each with `// TODO`)
- [ ] Run `cargo test -p codegen` — confirm helpers tests pass

---

```yaml
task:
  id: 25
  description: "Implement domain_generator: generate aggregate + commands + queries files"
  skill: test-driven-development
  parallel: false
  depends_on: [24]
  steps:
    - "Implement generate_domain(entity, output_dir) in tools/codegen/src/domain_generator.rs"
    - "Write test: generate Member domain → verify file contents contain correct type names"
    - "Run cargo test -p codegen"
  expected_output: "domain_generator tests pass; generated files contain correct PascalCase entity names"
  verification_cmd: "cargo test -p codegen"
  blockers: []
```

**T25 steps:**
- [ ] Implement `tools/codegen/src/domain_generator.rs`:
  ```rust
  use std::fs;
  use std::path::Path;
  use crate::helpers::{to_pascal_case, to_snake_case};

  pub fn generate(entity: &str, output_dir: &Path) {
    let pascal = to_pascal_case(entity);
    let snake = to_snake_case(&pascal);

    // aggregate
    let aggregate = format!(r#"use serde::{{Deserialize, Serialize}};

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct {pascal} {{
    pub id: i32,
    // TODO: add fields
  }}

  impl {pascal} {{
    pub fn new(id: i32) -> Self {{
      Self {{ id }}
    }}
  }}
  "#, pascal = pascal);

    let commands = format!(r#"use serde::{{Deserialize, Serialize}};

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Create{pascal}Command {{
    // TODO: add fields
  }}

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Update{pascal}Command {{
    pub id: i32,
  }}

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Delete{pascal}Command {{
    pub id: i32,
  }}
  "#, pascal = pascal);

    let queries = format!(r#"use serde::{{Deserialize, Serialize}};

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct GetAll{pascal}Query;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Get{pascal}ByIdQuery {{
    pub id: i32,
  }}

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct List{pascal} {{
    pub page: Option<u64>,
    pub page_size: Option<u64>,
  }}
  "#, pascal = pascal);

    let dir = output_dir.join("domain/src");
    fs::create_dir_all(dir.join("aggregate")).unwrap();
    fs::create_dir_all(dir.join("commands")).unwrap();
    fs::create_dir_all(dir.join("queries")).unwrap();
    fs::write(dir.join(format!("aggregate/{}.rs", snake)), aggregate).unwrap();
    fs::write(dir.join(format!("commands/{}.rs", snake)), commands).unwrap();
    fs::write(dir.join(format!("queries/{}.rs", snake)), queries).unwrap();
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use std::env;

    #[test]
    fn generates_aggregate_with_correct_name() {
      let tmp = env::temp_dir().join("codegen_test_domain");
      generate("member", &tmp);
      let content = fs::read_to_string(
        tmp.join("domain/src/aggregate/member.rs")
      ).unwrap();
      assert!(content.contains("pub struct Member"));
      fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn generates_commands_with_correct_name() {
      let tmp = env::temp_dir().join("codegen_test_domain2");
      generate("book_loan", &tmp);
      let content = fs::read_to_string(
        tmp.join("domain/src/commands/book_loan.rs")
      ).unwrap();
      assert!(content.contains("pub struct CreateBookLoanCommand"));
      fs::remove_dir_all(&tmp).ok();
    }
  }
  ```
- [ ] Run `cargo test -p codegen` — confirm GREEN

---

```yaml
task:
  id: 26
  description: "Implement app_generator and infra_generator"
  skill: null
  parallel: false
  depends_on: [25]
  steps:
    - "Implement generate() in app_generator.rs: service trait + command/query handler stubs"
    - "Implement generate() in infra_generator.rs: repository + service impl stubs"
    - "Write tests for each"
    - "Run cargo test -p codegen"
  expected_output: "app and infra generators produce correctly named files"
  verification_cmd: "cargo test -p codegen"
  blockers: []
```

**T26 steps:**
- [ ] Implement `tools/codegen/src/app_generator.rs` — `generate(entity, output_dir)` writes:
  - `application/src/{snake}/service.rs` — `{Pascal}Service` trait with `get_all`, `get_by_id`, `create`, `update`, `delete` stubs
  - `application/src/{snake}/command_handlers/create_handler.rs` — `Create{Pascal}CommandHandler` struct + handler trait
  - `application/src/{snake}/command_handlers/update_handler.rs`
  - `application/src/{snake}/command_handlers/delete_handler.rs`
  - `application/src/{snake}/query_handlers/get_all_handler.rs`
  - `application/src/{snake}/query_handlers/get_by_id_handler.rs`
  - `application/src/{snake}/query_handlers/list_handler.rs`
  - Each file uses `format!()` with `pascal` / `snake` substitutions on a string template
- [ ] Implement `tools/codegen/src/infra_generator.rs` — writes:
  - `infrastructure/src/repositories/{snake}_repository.rs` — `{Pascal}Repository` with SeaORM stubs
  - `infrastructure/src/services/{snake}_service.rs` — `{Pascal}ServiceImpl` implementing `{Pascal}Service`
- [ ] Add tests verifying file names and content contain the entity name
- [ ] Run `cargo test -p codegen` — confirm GREEN

---

```yaml
task:
  id: 27
  description: "Implement api_generator and wire main() CLI entry point"
  skill: null
  parallel: false
  depends_on: [26]
  steps:
    - "Implement api_generator.rs: 6 endpoint files + mod.rs"
    - "Wire main() to call all generators when given entity name arg"
    - "Run cargo run -p codegen -- Member and verify file tree"
    - "Run cargo test -p codegen"
  expected_output: "Running codegen Member creates full scaffold across all 5 layers"
  verification_cmd: "cargo test -p codegen && cargo run -p codegen -- TestEntity 2>&1 | grep Generating"
  blockers: []
```

**T27 steps:**
- [ ] Implement `tools/codegen/src/api_generator.rs` — writes:
  - `api/src/endpoints/{snake}/mod.rs` — registers all routes
  - `api/src/endpoints/{snake}/create.rs`
  - `api/src/endpoints/{snake}/update.rs`
  - `api/src/endpoints/{snake}/delete.rs`
  - `api/src/endpoints/{snake}/get_all.rs`
  - `api/src/endpoints/{snake}/get_by_id.rs`
  - `api/src/endpoints/{snake}/list.rs`
- [ ] Update `tools/codegen/src/main.rs`:
  ```rust
  fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
      eprintln!("Usage: codegen <EntityName>");
      std::process::exit(1);
    }
    let entity = &args[1];
    let output_dir = std::path::Path::new(".");
    println!("Generating scaffold for: {}", entity);

    domain_generator::generate(entity, output_dir);
    app_generator::generate(entity, output_dir);
    infra_generator::generate(entity, output_dir);
    api_generator::generate(entity, output_dir);

    println!("Done. Add the new files to your crate mod.rs files.");
  }
  ```
- [ ] Run `cargo test -p codegen` — confirm GREEN
- [ ] Run `cargo run -p codegen -- TestEntity` — confirm files are generated
- [ ] Commit: `git add -A && git commit -m "Implement codegen scaffold tool for new domain entities"`

---

## Execution Order Summary

```
T1–T7  (bug fixes — T2–T7 parallel, T1 serial)
  └─→ T8  (AppError NotFound)
        └─→ T9  (ApiError)
              └─→ T10 (rejection handler)
                    └─→ T11 (update endpoints)
                          └─→ T12 (get_available route)
                                └─→ T13 (search route)
                                      └─→ T14 (commands)
                                            └─→ T15 (service trait)
                                                  └─→ T16 (service impl)
                                                        ├─→ T17 (borrow handler)
                                                        └─→ T18 (return handler)
                                                              └─→ T19 (routes)
                                                                    └─→ T20 (test setup)
                                                                          ├─→ T21 (unit cmd)
                                                                          └─→ T22 (unit qry)
                                                                                └─→ T23 (integration)

T7 ──→ T24 (codegen setup) → T25 (domain gen) → T26 (app+infra gen) → T27 (api gen + main)
```

Parallel groups: {T2–T7}, {T17, T18}, {T21, T22}
