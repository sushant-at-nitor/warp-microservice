use std::sync::Arc;

use warp::Filter;

use application::books::{
  BookService, GetAvailableBooksQueryHandler,
  GetAvailableBooksQueryHandlerTrait,
};

use crate::errors::ApiError;

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
) -> impl Filter<
  Extract = (Arc<GetAvailableBooksQueryHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
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
