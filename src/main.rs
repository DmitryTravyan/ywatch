extern crate env_logger;
mod args;
mod server;
mod state;
mod client;

use std::time::Duration;
use futures::join;
use async_std::sync::Arc;
use futures::lock::Mutex;


const VERSION: &str = "0.1.0";

#[tokio::main]
async fn main() {
    env_logger::Builder::default().init();

    let state = Arc::new(state::State::init());
    let youtube_state = Arc::new(
        Mutex::new(state::youtube::Summary::new())
    );

    let configuration = args::Configuration::from_args(
        args::server::collect_args()
    );

    let server = server::new_server(
        &configuration, state.clone()
    );
    let client = client::new_client(youtube_state.clone());

    join!(server, client);
}
