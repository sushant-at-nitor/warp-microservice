pub mod startup;

use std::sync::Arc;

use application::books::{
  CreateBookCommandHandler, CreateBookCommandHandlerTrait,
};
use domain::commands::CreateBookCommand;
use env_logger::Env;
use infrastructure::{repositories::BookRepository, services::BookServiceImpl};
use log::info;

#[tokio::main]
async fn main() {
  // Load .env file
  dotenvy::dotenv().ok();

  env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
    .init();

  info!(
    "Logger initialized with RUST_LOG={}",
    std::env::var("RUST_LOG").unwrap_or_default()
  );

  let state = startup::build_app_state().await;
  // let api = endpoints::register(Arc::new(state));

  info!(
    "🚀 API running at http://localhost:{}",
    std::env::var("PORT").unwrap_or_else(|_| "3030".to_string())
  );

  let book_repo = BookRepository::new(state.db.connection.clone());
  let book_service = Arc::from(BookServiceImpl::new(book_repo));

  let create_book_command = CreateBookCommand {
    author: "Vivekanand".into(),
    title: "Rajyog".into(),
    isbn: "some-rand-isbn-1".into(),
    total_copies: 650,
    published_year: Some(1908),
  };

  let handler = CreateBookCommandHandler::new(book_service);
  match handler.handle(create_book_command).await {
    Ok(_) => println!("book created!"),
    Err(err) => println!("application error, {:?}", err),
  }
  // warp::serve(api)
  //   .run((
  //     [127, 0, 0, 1],
  //     std::env::var("PORT")
  //       .unwrap_or_else(|_| "3030".to_string())
  //       .parse()
  //       .unwrap_or(3030),
  //   ))
  //   .await;
}
