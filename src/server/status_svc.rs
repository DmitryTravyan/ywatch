use crate::state::State;
use hyper::http::{Request, Response, StatusCode};
use hyper::Body;
use std::sync::Arc;

pub(crate) async fn status(
    state: Arc<State>
) -> super::Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(state.get_json_as_str().into())
        .unwrap()
    )
}
