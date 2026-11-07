use std::sync::Arc;
use warp::Filter;
use application::member::{
  MemberService, CreateMemberCommandHandler, CreateMemberCommandHandlerTrait,
};
use domain::commands::CreateMemberCommand;
use crate::errors::ApiError;

pub fn route<T>(
  handler: Arc<CreateMemberCommandHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where T: MemberService + Send + Sync + 'static,
{
  warp::path!("member")
    .and(warp::post())
    .and(warp::any().map(move || handler.clone()))
    .and(warp::body::json())
    .and_then(|handler: Arc<CreateMemberCommandHandler<T>>, cmd: CreateMemberCommand| async move {
      match handler.handle(cmd).await {
        Ok(item) => Ok(warp::reply::json(&item)),
        Err(e) => Err(warp::reject::custom(ApiError::from(e))),
      }
    })
}
