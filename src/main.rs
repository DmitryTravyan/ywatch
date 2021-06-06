extern crate env_logger;
mod args;
mod server;
mod state;

use std::time::Duration;
use futures::join;

const VERSION: &str = "0.1.0";

#[tokio::main]
async fn main() {
    env_logger::Builder::default().init();

    let state = state::State::init();
    let configuration = args::Configuration::from_args(
        args::server::collect_args()
    );

    let server = server::new_server(&configuration, &state);

    join!(server);
}
