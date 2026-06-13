use std::error::Error;

use rumqttc::{AsyncClient, Event, Incoming, MqttOptions};

const BROKER_HOST: &str = "192.168.1.97";
const BROKER_PORT: u16 = 1883;
const CLIENT_ID: &str = "tcp-check-client";
const KEEP_ALIVE_SECS: u64 = 5;
const TOPIC: &str = "test/hello";
const PAYLOAD: &[u8] = b"hello from tcp client";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut options = MqttOptions::new(CLIENT_ID, BROKER_HOST, BROKER_PORT);
    options.set_keep_alive(std::time::Duration::from_secs(KEEP_ALIVE_SECS));

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
                println!("Connected to {BROKER_HOST}:{BROKER_PORT} (TCP)");
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
