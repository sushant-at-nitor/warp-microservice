use crate::errors::ApiError;
use std::sync::Arc;
use warp::Filter;
use application::member::{
  MemberService, UpdateMemberCommandHandler, UpdateMemberCommandHandlerTrait,
};
use domain::commands::UpdateMemberCommand;

pub fn route<T>(
  handler: Arc<UpdateMemberCommandHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: MemberService + Send + Sync + 'static,
{
  warp::path!("member" / i32)
    .and(warp::put())
    .and(with_handler(handler))
    .and(warp::body::json())
    .and_then(handle_update_member)
}

fn with_handler<T>(
  handler: Arc<UpdateMemberCommandHandler<T>>,
) -> impl Filter<
  Extract = (Arc<UpdateMemberCommandHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: MemberService + Send + Sync + 'static,
{
  warp::any().map(move || handler.clone())
}

async fn handle_update_member<T>(
  member_id: i32,
  handler: Arc<UpdateMemberCommandHandler<T>>,
  mut cmd: UpdateMemberCommand,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: MemberService + Send + Sync + 'static,
{
  cmd.id = member_id;

  match handler.handle(cmd).await {
    Ok(item) => Ok(warp::reply::json(&item)),
    Err(e) => Err(warp::reject::custom(ApiError::from(e))),
  }
}
