use hyper::http::Uri;
use hyper::Client;
use tokio::time::sleep;
use std::time::Duration;
use async_std::sync::{Arc};
use futures::lock::Mutex;
use crate::state::youtube::Summary;
use tokio::sync::mpsc::Sender;

pub(crate) async fn new_client(tx: Sender<u64>) {
    let uri: Uri = "http://google.com".parse().expect("Ok");
    let client = Client::new();
    loop {
        let mut resp = client.get(uri.clone()).await.expect("Nok");
        tx.send(1).await.unwrap();
        println!("Response {}", resp.status());
        sleep(Duration::from_secs(5)).await;
    }
}
