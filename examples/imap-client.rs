use std::time::Duration;

use imap_client::Client;
use tokio::time::sleep;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap())
        .with(fmt::layer())
        .init();

    let mut count = 0;

    loop {
        Client::tls("outlook.office365.com", 993).await.unwrap();
        sleep(Duration::from_millis(500)).await;

        count += 1;
        println!("=====> {count}");
    }
}
