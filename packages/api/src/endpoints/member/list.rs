use std::sync::Arc;
use warp::Filter;
use application::member::{
  MemberService, ListMemberQueryHandler, ListMemberQueryHandlerTrait,
};
use crate::errors::ApiError;
use domain::queries::ListMember;

pub fn route<T>(
  handler: Arc<ListMemberQueryHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: MemberService + Send + Sync + 'static,
{
  warp::path!("member" / "list")
    .and(warp::get())
    .and(warp::query::<ListMember>())
    .and(with_handler(handler))
    .and_then(handle_list_member)
}

fn with_handler<T>(
  handler: Arc<ListMemberQueryHandler<T>>,
) -> impl Filter<
  Extract = (Arc<ListMemberQueryHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: MemberService + Send + Sync + 'static,
{
  warp::any().map(move || handler.clone())
}

async fn handle_list_member<T>(
  query: ListMember,
  handler: Arc<ListMemberQueryHandler<T>>,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: MemberService + Send + Sync + 'static,
{
  match handler.handle(query).await {
    Ok(items) => Ok(warp::reply::json(&items)),
    Err(e) => Err(warp::reject::custom(ApiError::from(e))),
  }
}
