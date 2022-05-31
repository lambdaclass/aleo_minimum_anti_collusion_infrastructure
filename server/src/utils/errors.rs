#[derive(Debug)]
pub struct TallyError;
impl warp::reject::Reject for TallyError {}

#[derive(Debug)]
pub struct DBError;
impl warp::reject::Reject for DBError {}
