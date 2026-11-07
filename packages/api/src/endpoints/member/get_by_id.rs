use crate::errors::ApiError;
use std::sync::Arc;
use warp::Filter;
use application::member::{
  MemberService, GetByIdMemberQueryHandler, GetByIdMemberQueryHandlerTrait,
};
use domain::queries::GetMemberByIdQuery;

pub fn route<T>(
  handler: Arc<GetByIdMemberQueryHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: MemberService + Send + Sync + 'static,
{
  warp::path!("member" / i32)
    .and(warp::get())
    .and(with_handler(handler))
    .and_then(handle_get_by_id_member)
}

fn with_handler<T>(
  handler: Arc<GetByIdMemberQueryHandler<T>>,
) -> impl Filter<
  Extract = (Arc<GetByIdMemberQueryHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: MemberService + Send + Sync + 'static,
{
  warp::any().map(move || handler.clone())
}

async fn handle_get_by_id_member<T>(
  id: i32,
  handler: Arc<GetByIdMemberQueryHandler<T>>,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: MemberService + Send + Sync + 'static,
{
  match handler.handle(GetMemberByIdQuery { id }).await {
    Ok(item) => Ok(warp::reply::json(&item)),
    Err(e) => Err(warp::reject::custom(ApiError::from(e))),
  }
}
