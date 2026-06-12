use std::collections::HashMap;
use std::net::SocketAddr;

use rumqttd::{Broker, Config, ConnectionSettings, RouterConfig, ServerSettings, TlsConfig};

const BROKER_ID: usize = 0;
const LISTEN_ADDR: &str = "0.0.0.0:8883";
const SERVER_NAME: &str = "mqtt-tls";
const CERT_PATH: &str = "certs/server.cert.pem";
const KEY_PATH: &str = "certs/server.key.pem";
const CONNECTION_TIMEOUT_MS: u16 = 6000;
const MAX_PAYLOAD_SIZE: usize = 2048;
const MAX_INFLIGHT_COUNT: usize = 100;
const NEXT_CONNECTION_DELAY_MS: u64 = 1;
const MAX_CONNECTIONS: usize = 1000;
const MAX_OUTGOING_PACKET_COUNT: u64 = 200;
const MAX_SEGMENT_SIZE: usize = 1024 * 1024;
const MAX_SEGMENT_COUNT: usize = 10;

fn build_config() -> Config {
    Config {
        id: BROKER_ID,
        router: RouterConfig {
            max_connections: MAX_CONNECTIONS,
            max_outgoing_packet_count: MAX_OUTGOING_PACKET_COUNT,
            max_segment_size: MAX_SEGMENT_SIZE,
            max_segment_count: MAX_SEGMENT_COUNT,
            custom_segment: None,
            initialized_filters: None,
            shared_subscriptions_strategy: rumqttd::Strategy::default(),
        },
        v4: Some(HashMap::from([(
            SERVER_NAME.to_string(),
            ServerSettings {
                name: SERVER_NAME.to_string(),
                listen: LISTEN_ADDR.parse::<SocketAddr>().unwrap(),
                tls: Some(TlsConfig::Rustls {
                    capath: None,
                    certpath: CERT_PATH.to_string(),
                    keypath: KEY_PATH.to_string(),
                }),
                next_connection_delay_ms: NEXT_CONNECTION_DELAY_MS,
                connections: ConnectionSettings {
                    connection_timeout_ms: CONNECTION_TIMEOUT_MS,
                    max_payload_size: MAX_PAYLOAD_SIZE,
                    max_inflight_count: MAX_INFLIGHT_COUNT,
                    auth: None,
                    external_auth: None,
                    dynamic_filters: false,
                },
            },
        )])),
        v5: None,
        ws: None,
        cluster: None,
        console: None,
        bridge: None,
        prometheus: None,
        metrics: None,
    }
}

fn main() {
    let config = build_config();
    let mut broker = Broker::new(config);

    broker.start().unwrap();
}
