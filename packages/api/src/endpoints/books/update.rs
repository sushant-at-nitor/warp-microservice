use common::server_error::ServerError;
use std::sync::Arc;
use warp::Filter;

use application::books::{
  BookService, UpdateBookCommandHandlerTrait, command_handlers::UpdateBookCommandHandler
};
use domain::commands::UpdateBookCommand;

pub fn route<T>(
  handler: Arc<UpdateBookCommandHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  // PUT /books/{book_id}
  let endpoint = warp::path!("books" / i32)
    .and(warp::put())
    .and(with_handler(handler))
    .and(warp::body::json())
    .and_then(handle_update_book);

  endpoint
}

fn with_handler<T>(
  handler: Arc<UpdateBookCommandHandler<T>>,
) -> impl Filter<
  Extract = (Arc<UpdateBookCommandHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  warp::any().map(move || handler.clone())
}

async fn handle_update_book<T>(
  book_id: i32,
  handler: Arc<UpdateBookCommandHandler<T>>,
  mut cmd: UpdateBookCommand,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: BookService + Send + Sync + 'static,
{
  cmd.id = book_id;

  match handler.handle(cmd).await {
    Ok(book) => Ok(warp::reply::json(&book)),
    Err(_) => Err(warp::reject::custom(ServerError)),
  }
}
