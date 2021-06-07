use hyper::http::{Response, StatusCode};
use hyper::Body;

pub(crate) async fn list(

) -> super::Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body("not implemented".into())
        .unwrap()
    )
}