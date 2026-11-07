use std::sync::Arc;

use serde::Deserialize;
use warp::Filter;

use application::books::{
  BookService, BorrowBookCommandHandler, BorrowBookCommandHandlerTrait,
};
use domain::commands::BorrowBookCommand;

use crate::errors::ApiError;

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
) -> impl Filter<
  Extract = (Arc<BorrowBookCommandHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
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
