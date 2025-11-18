pub mod books;

use crate::startup::AppState;
use std::sync::Arc;
use warp::Filter;

pub fn register(
  state: Arc<AppState>,
) -> impl Filter<Extract = impl warp::Reply> + Clone {
  books::register(state)
}
