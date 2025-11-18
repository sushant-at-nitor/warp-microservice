mod create;
mod delete;
mod get_all;
mod get_by_id;
mod update;

use std::sync::Arc;
use warp::Filter;

use application::books::{
  command_handlers::CreateBookCommandHandler,
  command_handlers::DeleteBookCommandHandler,
  command_handlers::UpdateBookCommandHandler,
  query_handlers::GetAllBooksQueryHandler,
  query_handlers::GetBookByIdQueryHandler,
};
use infrastructure::{
  repositories::book_repository::BookRepository,
  services::book_service::BookServiceImpl,
};

use crate::startup::AppState;

pub fn register(
  state: Arc<AppState>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  let repo = BookRepository::new(state.db.connection.clone());
  let service = Arc::new(BookServiceImpl::new(repo));
  let create_handler = Arc::new(CreateBookCommandHandler::new(service.clone()));
  let update_handler = Arc::new(UpdateBookCommandHandler::new(service.clone()));
  let delete_handler = Arc::new(DeleteBookCommandHandler::new(service.clone()));
  let get_all_handler = Arc::new(GetAllBooksQueryHandler::new(service.clone()));
  let get_by_id_handler =
    Arc::new(GetBookByIdQueryHandler::new(service.clone()));

  // Each route is self-contained under /books
  let create = create::route(create_handler);
  let update = update::route(update_handler);
  let delete = delete::route(delete_handler);
  let get_all = get_all::route(get_all_handler);
  let get_by_id = get_by_id::route(get_by_id_handler);

  let routes = create.or(update).or(delete).or(get_all).or(get_by_id);

  routes
}
