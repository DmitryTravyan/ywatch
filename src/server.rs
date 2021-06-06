mod aggregator;
mod status_svc;
mod index_svc;

use crate::args::Configuration;
use crate::state::State;
use hyper::Server;
use tokio::net::unix::SocketAddr;
use hyper::service::{make_service_fn, service_fn};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

pub(crate) async fn new_server(conf: &Configuration, state: &State) {
    let addr = format!("{}:{}", conf.ip(), conf.port()).parse().unwrap();

    let state_ref = state.clone();
    let service = make_service_fn(move |_conn| {
        async {
            Ok::<_, GenericError>(service_fn(|req| {
                aggregator::api(req, state_ref.clone())
            }))
        }
    });
    let server = Server::bind(&addr)
        .serve(service);

    server.await.expect("nok");
}