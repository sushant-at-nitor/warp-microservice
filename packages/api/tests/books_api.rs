use std::sync::Arc;

use api::{endpoints, startup::AppState};
use database::{Database, migrations::Migrator};
use sea_orm_migration::MigratorTrait;
use serde_json::{Value, json};

// ── Test helpers ─────────────────────────────────────────────────────────────

async fn setup() -> impl warp::Filter<Extract = impl warp::Reply> + Clone {
  let conn = sea_orm::Database::connect("sqlite::memory:")
    .await
    .expect("in-memory SQLite failed");

  Migrator::up(&conn, None).await.expect("migration failed");

  let db = Database { connection: conn };
  let state = Arc::new(AppState { db });
  endpoints::register(state)
}

async fn create_test_book(
  filter: &(impl warp::Filter<Extract = impl warp::Reply> + Clone + 'static),
) -> Value {
  let res = warp::test::request()
    .method("POST")
    .path("/books")
    .json(&json!({
      "title": "Test Book",
      "author": "Test Author",
      "isbn": "978-3-16-148410-0",
      "total_copies": 3,
      "published_year": 2024
    }))
    .reply(filter)
    .await;
  assert_eq!(res.status(), 200);
  serde_json::from_slice(res.body()).unwrap()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn create_book_returns_200() {
  let filter = setup().await;
  let body: Value = create_test_book(&filter).await;
  assert_eq!(body["title"], "Test Book");
  assert_eq!(body["author"], "Test Author");
  assert_eq!(body["total_copies"], 3);
  assert_eq!(body["available_copies"], 3);
}

#[tokio::test]
async fn get_all_books_returns_200() {
  let filter = setup().await;
  create_test_book(&filter).await;

  let res = warp::test::request()
    .method("GET")
    .path("/books")
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 200);
  let books: Value = serde_json::from_slice(res.body()).unwrap();
  assert!(books.as_array().unwrap().len() >= 1);
}

#[tokio::test]
async fn get_book_by_id_returns_200() {
  let filter = setup().await;
  let book = create_test_book(&filter).await;
  let id = book["id"].as_i64().unwrap();

  let res = warp::test::request()
    .method("GET")
    .path(&format!("/books/{}", id))
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 200);
  let found: Value = serde_json::from_slice(res.body()).unwrap();
  assert_eq!(found["id"], id);
}

#[tokio::test]
async fn get_book_by_id_returns_404_when_missing() {
  let filter = setup().await;

  let res = warp::test::request()
    .method("GET")
    .path("/books/99999")
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn update_book_returns_200() {
  let filter = setup().await;
  let book = create_test_book(&filter).await;
  let id = book["id"].as_i64().unwrap();

  let res = warp::test::request()
    .method("PUT")
    .path(&format!("/books/{}", id))
    .json(&json!({ "title": "Updated Title" }))
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 200);
  let updated: Value = serde_json::from_slice(res.body()).unwrap();
  assert_eq!(updated["title"], "Updated Title");
}

#[tokio::test]
async fn update_book_returns_404_when_missing() {
  let filter = setup().await;

  let res = warp::test::request()
    .method("PUT")
    .path("/books/99999")
    .json(&json!({ "title": "No Such Book" }))
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn delete_book_returns_200() {
  let filter = setup().await;
  let book = create_test_book(&filter).await;
  let id = book["id"].as_i64().unwrap();

  let res = warp::test::request()
    .method("DELETE")
    .path(&format!("/books/{}", id))
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn delete_book_returns_404_when_missing() {
  let filter = setup().await;

  let res = warp::test::request()
    .method("DELETE")
    .path("/books/99999")
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn get_available_books_returns_200() {
  let filter = setup().await;
  create_test_book(&filter).await;

  let res = warp::test::request()
    .method("GET")
    .path("/books/available")
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 200);
  let books: Value = serde_json::from_slice(res.body()).unwrap();
  assert!(books.as_array().unwrap().len() >= 1);
}

#[tokio::test]
async fn search_books_by_author_returns_200() {
  let filter = setup().await;
  create_test_book(&filter).await;

  let res = warp::test::request()
    .method("GET")
    .path("/books/search?author=Test+Author")
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 200);
  let books: Value = serde_json::from_slice(res.body()).unwrap();
  assert!(books.as_array().unwrap().len() >= 1);
}

#[tokio::test]
async fn borrow_book_returns_200() {
  let filter = setup().await;
  let book = create_test_book(&filter).await;
  let id = book["id"].as_i64().unwrap();

  let res = warp::test::request()
    .method("POST")
    .path(&format!("/books/{}/borrow", id))
    .json(&json!({ "member_id": 42 }))
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 200);
  let borrowed: Value = serde_json::from_slice(res.body()).unwrap();
  assert_eq!(borrowed["available_copies"], 2);
}

#[tokio::test]
async fn borrow_book_returns_404_when_missing() {
  let filter = setup().await;

  let res = warp::test::request()
    .method("POST")
    .path("/books/99999/borrow")
    .json(&json!({ "member_id": 42 }))
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn borrow_book_returns_422_when_no_copies() {
  let filter = setup().await;
  let book = create_test_book(&filter).await;
  let id = book["id"].as_i64().unwrap();

  // Borrow all 3 copies
  for _ in 0..3 {
    let res = warp::test::request()
      .method("POST")
      .path(&format!("/books/{}/borrow", id))
      .json(&json!({ "member_id": 42 }))
      .reply(&filter)
      .await;
    assert_eq!(res.status(), 200);
  }

  // 4th borrow should fail
  let res = warp::test::request()
    .method("POST")
    .path(&format!("/books/{}/borrow", id))
    .json(&json!({ "member_id": 42 }))
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 422);
}

#[tokio::test]
async fn return_book_increases_available_copies() {
  let filter = setup().await;
  let book = create_test_book(&filter).await;
  let id = book["id"].as_i64().unwrap();

  // Borrow first
  warp::test::request()
    .method("POST")
    .path(&format!("/books/{}/borrow", id))
    .json(&json!({ "member_id": 42 }))
    .reply(&filter)
    .await;

  // Return
  let res = warp::test::request()
    .method("POST")
    .path(&format!("/books/{}/return", id))
    .json(&json!({ "member_id": 42 }))
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 200);
  let returned: Value = serde_json::from_slice(res.body()).unwrap();
  assert_eq!(returned["available_copies"], 3);
}

#[tokio::test]
async fn list_books_with_pagination_returns_200() {
  let filter = setup().await;
  create_test_book(&filter).await;

  let res = warp::test::request()
    .method("GET")
    .path("/books/list?page=1&page_size=10")
    .reply(&filter)
    .await;
  assert_eq!(res.status(), 200);
  let paged: Value = serde_json::from_slice(res.body()).unwrap();
  assert!(paged["total"].as_u64().unwrap() >= 1);
}
