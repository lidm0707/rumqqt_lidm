# rumqqt_lidm

MQTT broker with TLS (rustls) powered by [rumqttd](https://github.com/bytebeamio/rumqtt).

## Architecture

```
Broker (CT101 - 192.168.1.97:8883)   <--TLS-->   Client (Mac - 192.168.1.114)
  rumqttd + rustls                                  rumqttc + rustls
  certs/server.cert.pem                             certs/ca.cert.pem
  certs/server.key.pem
```

## Certificates

```
certs/
├── ca.cert.pem       # CA certificate (client trusts this)
├── ca.key.pem        # CA private key (signs server cert)
├── server.cert.pem   # Server cert (broker presents this)
└── server.key.pem    # Server private key
```

Server cert SAN: `IP:192.168.1.97`, `IP:192.168.1.114`, `DNS:localhost`

## Usage

### Start broker (on CT101)

```sh
cargo run --release
```

Listens on `0.0.0.0:8883` (MQTT over TLS).

### Run client (from Mac)

```sh
cargo run --example tls_client
```

Connects to `192.168.1.97:8883`, subscribes to `test/hello`, publishes a message.

### Deploy to CT101

```sh
scp certs/server.cert.pem certs/server.key.pem root@192.168.1.97:~/rumqqt_lidm/certs/
scp src/main.rs root@192.168.1.97:~/rumqqt_lidm/src/main.rs
```

### Regenerate certificates

```sh
# Server cert signed by CA
openssl genrsa -out certs/server.key.pem 2048
openssl req -new -key certs/server.key.pem -out /tmp/server.csr -subj "/CN=192.168.1.97"
openssl x509 -req -in /tmp/server.csr \
  -CA certs/ca.cert.pem -CAkey certs/ca.key.pem -CAcreateserial \
  -out certs/server.cert.pem -days 365 \
  -extfile <(echo "basicConstraints=CA:FALSE"; echo "keyUsage=digitalSignature,keyEncipherment"; echo "extendedKeyUsage=serverAuth"; echo "subjectAltName=IP:192.168.1.97,IP:192.168.1.114,DNS:localhost")
```

## Broker Config

| Setting | Value |
|---|---|
| Protocol | MQTT v4 |
| Listen | `0.0.0.0:8883` |
| TLS | rustls |
| Max connections | 1000 |
| Max payload | 2048 bytes |
| Connection timeout | 6s |

## Dependencies

- `rumqttd` 0.20.0 — MQTT broker
- `rumqttc` 0.25.1 — MQTT client (example)
- `tokio-rustls` — TLS (via rumqttc re-export)
- `rustls-pemfile` 2 — PEM cert loading
