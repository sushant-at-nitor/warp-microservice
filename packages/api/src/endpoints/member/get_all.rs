use std::sync::Arc;
use warp::Filter;
use application::member::{
  MemberService, GetAllMemberQueryHandler, GetAllMemberQueryHandlerTrait,
};
use crate::errors::ApiError;

pub fn route<T>(
  handler: Arc<GetAllMemberQueryHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where T: MemberService + Send + Sync + 'static,
{
  warp::path!("member")
    .and(warp::get())
    .and(warp::any().map(move || handler.clone()))
    .and_then(|handler: Arc<GetAllMemberQueryHandler<T>>| async move {
      match handler.handle().await {
        Ok(items) => Ok(warp::reply::json(&items)),
        Err(e) => Err(warp::reject::custom(ApiError::from(e))),
      }
    })
}
