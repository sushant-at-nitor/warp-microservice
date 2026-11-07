use std::sync::Arc;

use warp::Filter;

use application::books::{
  BookService, SearchBooksQueryHandler, SearchBooksQueryHandlerTrait,
};
use domain::queries::BookQuery;

use crate::errors::ApiError;

pub fn route<T>(
  handler: Arc<SearchBooksQueryHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  warp::path!("books" / "search")
    .and(warp::get())
    .and(warp::query::<BookQuery>())
    .and(with_handler(handler))
    .and_then(handle_search)
}

fn with_handler<T>(
  handler: Arc<SearchBooksQueryHandler<T>>,
) -> impl Filter<
  Extract = (Arc<SearchBooksQueryHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  warp::any().map(move || handler.clone())
}

async fn handle_search<T>(
  query: BookQuery,
  handler: Arc<SearchBooksQueryHandler<T>>,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: BookService + Send + Sync + 'static,
{
  match handler.handle(query).await {
    Ok(books) => Ok(warp::reply::json(&books)),
    Err(e) => Err(warp::reject::custom(ApiError::from(e))),
  }
}
