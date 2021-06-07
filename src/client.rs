use hyper::http::Uri;
use hyper::Client;
use tokio::time::sleep;
use std::time::Duration;
use async_std::sync::{Arc};
use futures::lock::Mutex;
use crate::state::youtube::Summary;

pub(crate) async fn new_client(youtube_state: Arc<Mutex<Summary>>) {
    let uri: Uri = "http://google.com".parse().expect("Ok");
    let client = Client::new();
    loop {
        {
            let unlocked_counter = youtube_state.lock().await;
            let mut resp = client.get(uri.clone()).await.expect("Nok");
            unlocked_counter.count();
            println!("Response {}", resp.status());
        }
        sleep(Duration::from_secs(5)).await;
    }
}
