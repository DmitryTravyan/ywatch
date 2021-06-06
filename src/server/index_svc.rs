use hyper::http::{Request, Response};
use hyper::Body;
use std::convert::Infallible;

pub(crate) async fn index(
    _: Request<Body>
) -> super::Result<Response<Body>> {
    Ok(Response::new(Body::from("{\n\t\"Hello\":\"World\"\n}")))
}
