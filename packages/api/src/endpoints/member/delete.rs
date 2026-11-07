use crate::errors::ApiError;
use std::sync::Arc;
use warp::Filter;
use application::member::{
  MemberService, DeleteMemberCommandHandler, DeleteMemberCommandHandlerTrait,
};
use domain::commands::DeleteMemberCommand;

pub fn route<T>(
  handler: Arc<DeleteMemberCommandHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: MemberService + Send + Sync + 'static,
{
  warp::path!("member" / i32)
    .and(warp::delete())
    .and(with_handler(handler))
    .and_then(handle_delete_member)
}

fn with_handler<T>(
  handler: Arc<DeleteMemberCommandHandler<T>>,
) -> impl Filter<
  Extract = (Arc<DeleteMemberCommandHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: MemberService + Send + Sync + 'static,
{
  warp::any().map(move || handler.clone())
}

async fn handle_delete_member<T>(
  id: i32,
  handler: Arc<DeleteMemberCommandHandler<T>>,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: MemberService + Send + Sync + 'static,
{
  match handler.handle(DeleteMemberCommand { id }).await {
    Ok(item) => Ok(warp::reply::json(&item)),
    Err(e) => Err(warp::reject::custom(ApiError::from(e))),
  }
}
