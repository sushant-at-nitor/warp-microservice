use crate::errors::ApiError;
use std::sync::Arc;
use warp::Filter;

use application::books::{
  BookService, CreateBookCommandHandler, CreateBookCommandHandlerTrait,
};
use domain::commands::CreateBookCommand;

pub fn route<T>(
  handler: Arc<CreateBookCommandHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  warp::path!("books")
    .and(warp::post())
    .and(with_handler(handler))
    .and(warp::body::json())
    .and_then(handle_create_book)
}

fn with_handler<T>(
  handler: Arc<CreateBookCommandHandler<T>>,
) -> impl Filter<
  Extract = (Arc<CreateBookCommandHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  warp::any().map(move || handler.clone())
}

async fn handle_create_book<T>(
  handler: Arc<CreateBookCommandHandler<T>>,
  cmd: CreateBookCommand,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: BookService + Send + Sync + 'static,
{
  match handler.handle(cmd).await {
    Ok(book) => Ok(warp::reply::json(&book)),
    Err(e) => Err(warp::reject::custom(ApiError::from(e))),
  }
}
