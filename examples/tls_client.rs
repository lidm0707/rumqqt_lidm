use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use rumqttc::tokio_rustls::rustls::{ClientConfig, RootCertStore};
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, Transport};

const BROKER_HOST: &str = "192.168.1.97";
const BROKER_PORT: u16 = 8883;
const CA_CERT_PATH: &str = "certs/ca.cert.pem";
const CLIENT_ID: &str = "tls-check-client";
const KEEP_ALIVE_SECS: u64 = 5;
const TOPIC: &str = "test/hello";
const PAYLOAD: &[u8] = b"hello from tls client";

fn load_ca_cert() -> Result<RootCertStore, Box<dyn Error>> {
    let mut store = RootCertStore::empty();
    let file = File::open(CA_CERT_PATH)?;
    let mut reader = BufReader::new(file);

    let certs = rustls_pemfile::certs(&mut reader).collect::<Result<Vec<_>, _>>()?;

    store.add_parsable_certificates(certs);
    Ok(store)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let root_store = load_ca_cert()?;

    let client_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let mut options = MqttOptions::new(CLIENT_ID, BROKER_HOST, BROKER_PORT);
    options.set_keep_alive(std::time::Duration::from_secs(KEEP_ALIVE_SECS));
    options.set_transport(Transport::tls_with_config(client_config.into()));

    let (client, mut eventloop) = AsyncClient::new(options, 10);

    client.subscribe(TOPIC, rumqttc::QoS::AtLeastOnce).await?;
    client
        .publish(TOPIC, rumqttc::QoS::AtLeastOnce, false, PAYLOAD)
        .await?;

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                println!("Topic: {}, Payload: {:?}", p.topic, p.payload);
            }
            Ok(Event::Incoming(Incoming::ConnAck(_))) => {
                println!("Connected to {BROKER_HOST}:{BROKER_PORT} (TLS)");
            }
            Ok(Event::Incoming(i)) => {
                println!("Incoming = {i:?}");
            }
            Ok(Event::Outgoing(o)) => {
                println!("Outgoing = {o:?}");
            }
            Err(e) => {
                eprintln!("Error = {e:?}");
                return Ok(());
            }
        }
    }
}
