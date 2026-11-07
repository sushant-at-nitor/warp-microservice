use crate::errors::ApiError;
use std::sync::Arc;
use warp::Filter;

use application::books::{
  BookService, DeleteBookCommandHandler, DeleteBookCommandHandlerTrait,
};
use domain::commands::DeleteBookCommand;

pub fn route<T>(
  handler: Arc<DeleteBookCommandHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  warp::path!("books" / i32)
    .and(warp::delete())
    .and(with_handler(handler))
    .and_then(handle_delete_book)
}

fn with_handler<T>(
  handler: Arc<DeleteBookCommandHandler<T>>,
) -> impl Filter<
  Extract = (Arc<DeleteBookCommandHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  warp::any().map(move || handler.clone())
}

async fn handle_delete_book<T>(
  id: i32,
  handler: Arc<DeleteBookCommandHandler<T>>,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: BookService + Send + Sync + 'static,
{
  match handler.handle(DeleteBookCommand { id }).await {
    Ok(book) => match book {
      Some(book) => Ok(warp::reply::json(&book)),
      None => {
        Err(warp::reject::custom(ApiError::NotFound(format!(
          "Book {} not found",
          id
        ))))
      }
    },
    Err(e) => Err(warp::reject::custom(ApiError::from(e))),
  }
}
