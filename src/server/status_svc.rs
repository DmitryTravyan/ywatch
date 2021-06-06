use crate::state::State;
use hyper::http::{Request, Response};
use hyper::Body;
use reqwest::StatusCode;

pub(crate) async fn status(
    state: &State
) -> super::Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(state.get_json_as_str().into())
        .unwrap()
    )
}
