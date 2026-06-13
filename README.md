# rumqqt_lidm

MQTT broker (plain TCP, no TLS) powered by [rumqttd](https://github.com/bytebeamio/rumqtt).

## Architecture

```
Broker (CT101 - 192.168.1.97:1883)   <--TCP-->   Client (Mac - 192.168.1.114)
  rumqttd                                          rumqttc
```

## Usage

### Start broker (on CT101)

```sh
cargo run --release
```

Listens on `0.0.0.0:1883` (plain MQTT over TCP).

### Run client (from Mac)

```sh
cargo run --example client
```

Connects to `192.168.1.97:1883`, subscribes to `test/hello`, publishes a message.

### Deploy to CT101

```sh
scp src/main.rs root@192.168.1.97:~/rumqqt_lidm/src/main.rs
```

No certificates required.

## Broker Config

| Setting | Value |
|---|---|
| Protocol | MQTT v4 |
| Listen | `0.0.0.0:1883` |
| TLS | none (plain TCP) |
| Max connections | 1000 |
| Max payload | 2048 bytes |
| Connection timeout | 6s |

## Dependencies

- `rumqttd` 0.20.0 — MQTT broker
- `rumqttc` 0.25.1 — MQTT client (example)
- `tokio` 1 — async runtime
- `tracing-subscriber` 0.3 — logging
