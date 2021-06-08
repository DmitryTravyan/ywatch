mod aggregator;
mod status_svc;
mod index_svc;
mod youtube_svc;

use crate::args::Configuration;
use crate::state::State;
use hyper::Server;
use tokio::net::unix::SocketAddr;
use hyper::service::{make_service_fn, service_fn};
use async_std::sync::Arc;
use futures::lock::Mutex;
use crate::state::youtube::Summary;
use crate::Counter;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

pub(crate) async fn new_server(
    conf: &Configuration,
    state: Arc<State>,
    counter: &Mutex<Counter>,
) {
    let addr = format!("{}:{}", conf.ip(), conf.port()).parse().unwrap();
    let service = make_service_fn(move |_conn| {
        let state_ref = state.clone();
        let counter_ref = counter.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                aggregator::api(req, state_ref.to_owned(), counter_ref.to_owned())
            }))
        }
    });
    let server = Server::bind(&addr)
        .serve(service);

    server.await.expect("nok");
}