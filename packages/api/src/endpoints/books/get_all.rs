use common::server_error::ServerError;
use std::sync::Arc;
use warp::Filter;

use application::books::{
  query_handlers::get_all_handler::{
    GetAllBooksQueryHandler, GetAllBooksQueryHandlerTrait,
  },
  service::BookService,
};
use domain::queries::GetAllBooksQuery;

pub fn route<T>(
  handler: Arc<GetAllBooksQueryHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  // GET /books
  warp::path!("books")
    .and(warp::get())
    .and(with_handler(handler))
    .and_then(handle_get_all_books)
}

fn with_handler<T>(
  handler: Arc<GetAllBooksQueryHandler<T>>,
) -> impl Filter<
  Extract = (Arc<GetAllBooksQueryHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  warp::any().map(move || handler.clone())
}

async fn handle_get_all_books<T>(
  handler: Arc<GetAllBooksQueryHandler<T>>,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: BookService + Send + Sync + 'static,
{
  let query = GetAllBooksQuery;
  match handler.handle(query).await {
    Ok(books) => Ok(warp::reply::json(&books)),
    Err(err) => Err(warp::reject::custom(ServerError)),
  }
}
