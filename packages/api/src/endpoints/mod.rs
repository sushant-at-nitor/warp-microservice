pub mod books;
pub mod health;

use crate::{rejection::handle_rejection, startup::AppState};
use std::sync::Arc;
use warp::Filter;

pub fn register(
  state: Arc<AppState>,
) -> impl Filter<Extract = impl warp::Reply> + Clone {
  let routes = books::register(state).or(health::register());
  routes.recover(handle_rejection)
}
