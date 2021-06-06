use crate::state::State;
use hyper::http::{Request, Response, StatusCode, Method};
use hyper::Body;

static NOTFOUND: &[u8] = b"Not Found";

pub(crate) async fn api(req: Request<Body>, state: &State) -> super::Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => {
            super::index_svc::index(req).await
        },
        (&Method::GET, "/api/v1/status") => {
            super::status_svc::status(state).await
        },
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap()
            )
        }
    }
}