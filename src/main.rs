use std::collections::HashMap;
use std::net::SocketAddr;

use rumqttd::{Broker, Config, ConnectionSettings, RouterConfig, ServerSettings, Strategy};

const BROKER_ID: usize = 0;
const LISTEN_ADDR: &str = "0.0.0.0:1883";
const LISTEN_ADDR_V5: &str = "0.0.0.0:1884";
const SERVER_NAME: &str = "mqtt-tcp";
const SERVER_NAME_V5: &str = "mqtt-tcp-v5";
const CONNECTION_TIMEOUT_MS: u16 = 6000;
const MAX_PAYLOAD_SIZE: usize = 2048;
const MAX_INFLIGHT_COUNT: usize = 100;
const NEXT_CONNECTION_DELAY_MS: u64 = 1;
const MAX_CONNECTIONS: usize = 1000;
const MAX_OUTGOING_PACKET_COUNT: u64 = 200;
const MAX_SEGMENT_SIZE: usize = 1024 * 1024;
const MAX_SEGMENT_COUNT: usize = 10;

fn server(name: &str, listen: &str) -> (String, ServerSettings) {
    (
        name.to_string(),
        ServerSettings {
            name: name.to_string(),
            listen: listen.parse::<SocketAddr>().unwrap(),
            tls: None,
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
    )
}

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
            shared_subscriptions_strategy: Strategy::default(),
        },
        v4: Some(HashMap::from([server(SERVER_NAME, LISTEN_ADDR)])),
        v5: Some(HashMap::from([server(SERVER_NAME_V5, LISTEN_ADDR_V5)])),
        ws: None,
        cluster: None,
        console: None,
        bridge: None,
        prometheus: None,
        metrics: None,
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = build_config();
    let mut broker = Broker::new(config);

    broker.start().unwrap();
}
