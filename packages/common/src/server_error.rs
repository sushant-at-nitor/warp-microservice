#[derive(Debug)]
pub struct ServerError;
impl warp::reject::Reject for ServerError {}
