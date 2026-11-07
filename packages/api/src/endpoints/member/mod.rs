mod create;
mod update;
mod delete;
mod get_all;
mod get_by_id;
mod list;

use std::sync::Arc;
use warp::Filter;
use application::member::{
  CreateMemberCommandHandler, UpdateMemberCommandHandler, DeleteMemberCommandHandler,
  GetAllMemberQueryHandler, GetByIdMemberQueryHandler, ListMemberQueryHandler,
};
use infrastructure::{
  repositories::member_repository::MemberRepository,
  services::member_service::MemberServiceImpl,
};
use crate::startup::AppState;

pub fn register(state: Arc<AppState>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  let repo = MemberRepository::new(state.db.connection.clone());
  let service = Arc::new(MemberServiceImpl::new(repo));
  let create_handler = Arc::new(CreateMemberCommandHandler::new(service.clone()));
  let update_handler = Arc::new(UpdateMemberCommandHandler::new(service.clone()));
  let delete_handler = Arc::new(DeleteMemberCommandHandler::new(service.clone()));
  let get_all_handler = Arc::new(GetAllMemberQueryHandler::new(service.clone()));
  let get_by_id_handler = Arc::new(GetByIdMemberQueryHandler::new(service.clone()));
  let list_handler = Arc::new(ListMemberQueryHandler::new(service.clone()));

  create::route(create_handler)
    .or(update::route(update_handler))
    .or(delete::route(delete_handler))
    .or(get_all::route(get_all_handler))
    .or(get_by_id::route(get_by_id_handler))
    .or(list::route(list_handler))
}
