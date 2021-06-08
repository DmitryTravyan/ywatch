use hyper::http::{Response, StatusCode};
use hyper::Body;
use crate::state::youtube::Summary;
use std::sync::Arc;
use futures::lock::Mutex;
use crate::Counter;

pub(crate) async fn list(
    counter: &Mutex<Counter>,
) -> super::Result<Response<Body>> {
    let counter_ref = counter.lock().await;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(counter_ref.get().to_string().into())
        .unwrap()
    )
}