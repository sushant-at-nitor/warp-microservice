use domain::aggregate::{book::Book, member::Member};
use utoipa::OpenApi;
use warp::Filter;

#[derive(OpenApi)]
#[openapi(
  info(
    title = "Warp Microservice API",
    version = env!("CARGO_PKG_VERSION"),
    description = "Library management API"
  ),
  components(schemas(Book, Member)),
  tags(
    (name = "books", description = "Book management"),
    (name = "members", description = "Member management"),
    (name = "health", description = "Health check")
  )
)]
pub struct ApiDoc;

pub fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("api-docs" / "openapi.json")
    .and(warp::get())
    .and_then(handle)
}

async fn handle() -> Result<impl warp::Reply, warp::Rejection> {
  Ok(warp::reply::json(&ApiDoc::openapi()))
}
