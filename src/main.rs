extern crate env_logger;
mod args;
mod server;
mod state;
mod client;

use std::time::Duration;
use futures::join;
use async_std::sync::Arc;
use futures::lock::Mutex;
use tokio::sync::mpsc::{Receiver, Sender};
use std::ops::AddAssign;


const VERSION: &str = "0.1.0";

#[tokio::main]
async fn main() {
    env_logger::Builder::default().init();

    let state = Arc::new(state::State::init());
    let (tx, rx):(Sender<u64>, Receiver<u64>) = tokio::sync::mpsc::channel(32);
    let counter = Mutex::new(Counter(0));

    let configuration = args::Configuration::from_args(
        args::server::collect_args()
    );

    let server = server::new_server(
        &configuration, state.clone(), &counter
    );
    let client = client::new_client(tx.clone());
    let count = count(rx, counter);

    join!(server, client, count);
}

async fn count(mut rx: Receiver<u64>, counter: Mutex<Counter>) {
    while let Some(v) = rx.recv().await {
        let mut cnt = counter.lock().await;
        cnt.count();
    }
}

struct Counter (i32);

impl Counter {
    fn count(&mut self) {
        self.0 = self.0 + 1;
    }
    fn get(&self) -> &i32 {
        &self.0
    }
}