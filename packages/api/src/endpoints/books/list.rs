use std::sync::Arc;
use warp::Filter;

use application::books::{
  BookService, ListBooksQueryHandler, ListBooksQueryHandlerTrait,
};
use common::server_error::ServerError;
use domain::queries::ListBooks;

pub fn route<T>(
  handler: Arc<ListBooksQueryHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  // GET /books
  warp::path!("books" / "list")
    .and(warp::get())
    .and(warp::query::<ListBooks>())
    .and(with_handler(handler))
    .and_then(handle_get_all_books)
}

fn with_handler<T>(
  handler: Arc<ListBooksQueryHandler<T>>,
) -> impl Filter<
  Extract = (Arc<ListBooksQueryHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  warp::any().map(move || handler.clone())
}

async fn handle_get_all_books<T>(
  query: ListBooks,
  handler: Arc<ListBooksQueryHandler<T>>,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: BookService + Send + Sync + 'static,
{
  match handler.handle(query).await {
    Ok(books) => Ok(warp::reply::json(&books)),
    Err(_) => Err(warp::reject::custom(ServerError)),
  }
}
