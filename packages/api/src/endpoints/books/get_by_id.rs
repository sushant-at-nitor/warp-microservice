use common::server_error::ServerError;
use std::sync::Arc;
use warp::Filter;

use application::books::{
  query_handlers::get_by_id_handler::{
    GetBookByIdQueryHandler, GetBookByIdQueryHandlerTrait,
  },
  service::BookService,
};
use domain::queries::GetBookByIdQuery;

pub fn route<T>(
  handler: Arc<GetBookByIdQueryHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  // GET /books/<i32>
  warp::path!("books" / i32)
    .and(warp::get())
    .and(with_handler(handler))
    .and_then(handle_get_by_id_book)
}

fn with_handler<T>(
  handler: Arc<GetBookByIdQueryHandler<T>>,
) -> impl Filter<
  Extract = (Arc<GetBookByIdQueryHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: BookService + Send + Sync + 'static,
{
  warp::any().map(move || handler.clone())
}

async fn handle_get_by_id_book<T>(
  id: i32,
  handler: Arc<GetBookByIdQueryHandler<T>>,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: BookService + Send + Sync + 'static,
{
  match handler.handle(GetBookByIdQuery { id }).await {
    Ok(book) => match book {
      Some(book) => Ok(warp::reply::json(&book)),
      None => {
        // return 404
        Err(warp::reject::not_found())
      }
    },
    Err(_) => Err(warp::reject::custom(ServerError)),
  }
}
