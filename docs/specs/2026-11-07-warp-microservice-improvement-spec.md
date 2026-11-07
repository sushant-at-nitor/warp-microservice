# Spec: warp-microservice Full Improvement Pipeline (2026-11-07)

**Goal:** Fix all known bugs, complete missing features, add error handling, borrow/return domain flow, a full test suite, and a domain scaffold codegen tool.

**Approved approaches:** See per-stage decisions below.

**Out of scope:** Member entity validation, OpenAPI spec generation, event sourcing / event store, authentication/authorisation.

---

## Stage 1 — Bug Fixes (7 items)

All fixes are unambiguous. No design variation.

| # | File | Fix |
|---|------|-----|
| 1 | `infrastructure/src/mappers/book.rs:12` | Map `available_copies` from `model.available_copies`, not `model.total_copies` |
| 2 | `api/src/endpoints/books/delete.rs:37` | Rename `handle_create_book` → `handle_delete_book` |
| 3 | `api/src/endpoints/books/update.rs:48` | Propagate `AppError` variant instead of collapsing to `ServerError` (fixed by Stage 2 error handling) |
| 4 | `api/src/endpoints/books/get_all.rs:47` | Suppress or use `err` variable in the `Err` arm |
| 5 | `api/Dockerfile:7` | Fix `COPY` paths — use `Cargo.toml`/`Cargo.lock` relative to context root |
| 6 | `database/src/database.rs:15` | Remove `println!` credential leak; use `log::debug!` after connecting (not before) |
| 7 | `api/src/main.rs:28` | Bind `[0, 0, 0, 0]` instead of `[127, 0, 0, 1]` |

**Additional cleanup (found during review):**
- Remove `sea-orm-cli = "1.1.0"` from `api/Cargo.toml` `[dependencies]` (it is a CLI tool, not a library)
- Remove dead commented-out code from `update_handler.rs` and `book_repository.rs`
- Remove dead commented-out code from `search_books` in `book_repository.rs`

---

## Stage 2 — Error Handling Architecture + Missing Endpoints

### 2a: Error Handling (ApiError wrapper)

**Approach B approved:** Keep `common` warp-free. Add an `ApiError` type in the `api` crate.

**Components:**
- `api/src/errors.rs` — `ApiError` enum with variants: `NotFound(String)`, `Validation(String)`, `Internal(String)`
- `ApiError` implements `warp::reject::Reject`
- `impl From<AppError> for ApiError` — maps `ValidationError` → `Validation`, `DatabaseError` → `Internal`
- `api/src/rejection.rs` (or inline in `startup.rs`) — `async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection>` returning JSON `{ "error": "...", "status": N }`
- `startup.rs` wires `.recover(handle_rejection)` onto the route filter

**HTTP status mapping:**
| `ApiError` variant | HTTP status |
|--------------------|------------|
| `NotFound` | 404 |
| `Validation` | 422 |
| `Internal` | 500 |
| `warp::reject::not_found()` | 404 |
| Unhandled | 500 |

**All existing endpoints updated** to return `Err(warp::reject::custom(ApiError::from(e)))` instead of `Err(warp::reject::custom(ServerError))`.

### 2b: Wire Missing Endpoints

Both service + repository implementations already exist — only routes and handlers are missing.

| Endpoint | Route | Handler |
|----------|-------|---------|
| `GET /books/available` | `warp::path!("books" / "available").and(warp::get())` | New `GetAvailableBooksQueryHandler` following existing query handler pattern |
| `GET /books/search` | `warp::path("books").and(warp::path("search")).and(warp::get()).and(warp::query::<BookQuery>())` | New `SearchBooksQueryHandler` following existing query handler pattern |

**New files:**
- `application/src/books/query_handlers/get_available_handler.rs`
- `application/src/books/query_handlers/search_handler.rs`
- `api/src/endpoints/books/get_available.rs`
- `api/src/endpoints/books/search.rs`

**Updated files:**
- `application/src/books/query_handlers/mod.rs` — re-export new handlers
- `api/src/endpoints/books/mod.rs` — register new routes

---

## Stage 3 — BorrowBook / ReturnBook Feature

**Approach A approved:** `POST /books/:id/borrow` + `POST /books/:id/return`

**member_id:** Trusted integer from request body. No member-existence validation.

### Domain layer (already exists — no changes needed)
- `Book::borrow_copy(member_id: i32) -> Result<(), &str>` ✓
- `Book::return_copy(member_id: i32)` ✓
- `BookBorrowed` / `BookReturned` domain events ✓

### Commands (new — domain crate)
```rust
// domain/src/commands/book.rs — append
pub struct BorrowBookCommand { pub book_id: i32, pub member_id: i32 }
pub struct ReturnBookCommand { pub book_id: i32, pub member_id: i32 }
```

### Service trait (new methods — application crate)
```rust
// application/src/books/service.rs — append
async fn borrow_book(&self, book_id: i32, member_id: i32) -> Result<Book, AppError>;
async fn return_book(&self, book_id: i32, member_id: i32) -> Result<Book, AppError>;
```

Note: These methods return `AppError` (not `DbErr`) because the borrow logic can fail with domain errors (`available_copies == 0` → `AppError::ValidationError`).

### Infrastructure (new methods)
`BookServiceImpl::borrow_book`:
1. `get_by_id(book_id)` → `AppError::NotFound` if None
2. `book.borrow_copy(member_id)` → `AppError::ValidationError("No copies available")` if Err
3. `service.update(book)` → `AppError::DatabaseError` if Err

`BookServiceImpl::return_book`:
1. `get_by_id(book_id)` → `AppError::NotFound` if None
2. `book.return_copy(member_id)`
3. `service.update(book)`

**New `NotFound` variant needed in `AppError`:**
```rust
pub enum AppError { DatabaseError(String), ValidationError(String), NotFound(String) }
```

### Application handlers (new files)
- `application/src/books/command_handlers/borrow_handler.rs`
- `application/src/books/command_handlers/return_handler.rs`

### API routes (new files)
- `api/src/endpoints/books/borrow.rs` — `POST /books/:id/borrow`, body: `{ member_id: i32 }`
- `api/src/endpoints/books/return_book.rs` — `POST /books/:id/return`, body: `{ member_id: i32 }`

---

## Stage 4 — Test Suite

**Strategy:** Two layers — unit tests (mock service) + integration tests (real SQLite in-process).

### Unit tests
Location: `#[cfg(test)]` modules within each handler file.

Scope: Command and query handlers. Mock `BookService` using a hand-rolled mock struct (no mockall dependency unless already present).

Test cases per handler:
- Happy path: correct response returned
- Error path: `AppError` correctly propagated

### Integration tests
Location: `packages/api/tests/books_api.rs` (Rust integration test convention).

Setup: Spin up warp routes with a real in-memory SQLite connection (`sqlite::memory:`). No server port needed — use `warp::test::request()`.

Test cases:
- `POST /books` → 200 with created book
- `GET /books` → 200 list
- `GET /books/:id` → 200 / 404
- `PUT /books/:id` → 200 / 404
- `DELETE /books/:id` → 200 / 404
- `GET /books/available` → 200
- `GET /books/search?author=...` → 200
- `POST /books/:id/borrow` → 200 / 422 (no copies) / 404
- `POST /books/:id/return` → 200 / 404

---

## Stage 5 — Codegen Tool (Domain Scaffold Generator)

**Approach A approved:** Rust string templates, no extra dependencies.

### CLI interface
```
cargo run -p codegen -- <EntityName>
# e.g.: cargo run -p codegen -- Member
```

Outputs files for the new entity across all 5 layers.

### Generator responsibilities

| File | Generator | Output |
|------|-----------|--------|
| `domain/src/aggregate/<entity>.rs` | `domain_generator.rs` | Aggregate struct + `new()` + `apply_update()` |
| `domain/src/commands/<entity>.rs` | `domain_generator.rs` | `Create<E>Command`, `Update<E>Command`, `Delete<E>Command` |
| `domain/src/queries/<entity>.rs` | `domain_generator.rs` | `GetAll<E>Query`, `GetById<E>Query`, `List<E>` |
| `application/src/<entity>/service.rs` | `app_generator.rs` | `<E>Service` trait with CRUD methods |
| `application/src/<entity>/command_handlers/` | `app_generator.rs` | 3 command handler files |
| `application/src/<entity>/query_handlers/` | `app_generator.rs` | 3 query handler files |
| `infrastructure/src/services/<entity>_service.rs` | `infra_generator.rs` | `<E>ServiceImpl` |
| `infrastructure/src/repositories/<entity>_repository.rs` | `infra_generator.rs` | `<E>Repository` |
| `api/src/endpoints/<entity>/` | `api_generator.rs` | 6 endpoint files + `mod.rs` |

### Template strategy
Each generator file exports a `generate(entity: &str, output_dir: &Path)` function.
Templates use `{{EntityName}}` (PascalCase), `{{entity_name}}` (snake_case) placeholders replaced via `str::replace`.

`helpers.rs` exports: `to_pascal_case(s: &str) -> String`, `to_snake_case(s: &str) -> String`.

`mod.rs` exports all generators and contains `main()` logic: parse args → call each generator → print created file list.

---

## Cross-cutting decisions

- `common::AppError` gains a `NotFound(String)` variant (Stage 3 requires it; Stage 2 error handler maps it to 404)
- `common::ServerError` and existing `ServerError`-based rejection handling are removed and replaced by `ApiError`
- All `println!` in production paths replaced with `log::info!` / `log::debug!` / `log::error!`
- Dead commented-out code removed throughout
