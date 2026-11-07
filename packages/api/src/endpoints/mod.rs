pub mod books;

use crate::{rejection::handle_rejection, startup::AppState};
use std::sync::Arc;
use warp::Filter;

pub fn register(
  state: Arc<AppState>,
) -> impl Filter<Extract = impl warp::Reply> + Clone {
  books::register(state).recover(handle_rejection)
}
