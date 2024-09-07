use std::{sync::Arc, time::Duration};

use once_cell::sync::Lazy;
use tokio::{io::AsyncReadExt, net::TcpStream, time::sleep};
use tokio_rustls::{
    rustls::{pki_types::ServerName, ClientConfig, RootCertStore},
    TlsConnector,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

static ROOT_CERT_STORE: Lazy<RootCertStore> = Lazy::new(|| {
    let mut root_store = RootCertStore::empty();

    for cert in rustls_native_certs::load_native_certs().unwrap() {
        root_store.add(cert).unwrap();
    }

    root_store
});

static HOST: &str = "outlook.office365.com";

#[tokio::main]
async fn main() {
    loop {
        let tcp_stream = TcpStream::connect((HOST, 993)).await.unwrap();

        let mut config = ClientConfig::builder()
            .with_root_certificates(ROOT_CERT_STORE.clone())
            .with_no_client_auth();

        config.alpn_protocols = vec![b"imap".to_vec()];

        let connector = TlsConnector::from(Arc::new(config));
        let dnsname = ServerName::try_from(HOST.to_string()).unwrap();

        let mut tls_stream = connector.connect(dnsname, tcp_stream).await.unwrap();

        let mut buf = Vec::new();
        tls_stream.read_buf(&mut buf).await.unwrap();

        println!("buf: {:?}", String::from_utf8_lossy(&buf));
        sleep(Duration::from_millis(500)).await;
    }
}