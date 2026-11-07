pub mod books;
pub mod health;
pub mod member;

use crate::{rejection::handle_rejection, startup::AppState};
use std::sync::Arc;
use warp::Filter;

pub fn register(
  state: Arc<AppState>,
) -> impl Filter<Extract = impl warp::Reply> + Clone {
  let routes = books::register(state.clone())
    .or(member::register(state))
    .or(health::register())
    .or(crate::openapi::register());
  routes.recover(handle_rejection)
}
