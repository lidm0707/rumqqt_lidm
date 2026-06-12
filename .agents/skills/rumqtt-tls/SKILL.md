---
name: rumqtt-tls
description: Set up rumqttd MQTT broker with TLS (rustls) and rumqttc client. Use when configuring the broker, managing certificates, deploying to CT101 (192.168.1.97), or debugging TLS connection issues.
---

# Rumqtt TLS Broker & Client

## Project Overview

MQTT v4 broker (`rumqttd`) with TLS via rustls, running on CT101 (192.168.1.97:8883). Client (`rumqttc`) connects from Mac (192.168.1.114).

## Key Files

- `src/main.rs` — broker config and startup
- `examples/tls_client.rs` — TLS client example
- `certs/ca.cert.pem` + `certs/ca.key.pem` — CA (signer, client trusts this)
- `certs/server.cert.pem` + `certs/server.key.pem` — server cert (broker presents this)

## Certificate Rules

### CA cert ≠ server cert

rustls rejects CA certificates (`basicConstraints: CA:TRUE`) as server certificates. The server cert must have:

- `basicConstraints = CA:FALSE`
- `keyUsage = digitalSignature, keyEncipherment`
- `extendedKeyUsage = serverAuth`
- `subjectAltName` with the broker IP(s)

### Generating a server cert

```sh
openssl genrsa -out certs/server.key.pem 2048
openssl req -new -key certs/server.key.pem -out /tmp/server.csr -subj "/CN=192.168.1.97"
openssl x509 -req -in /tmp/server.csr \
  -CA certs/ca.cert.pem -CAkey certs/ca.key.pem -CAcreateserial \
  -out certs/server.cert.pem -days 365 \
  -extfile <(printf "basicConstraints=CA:FALSE\nkeyUsage=digitalSignature,keyEncipherment\nextendedKeyUsage=serverAuth\nsubjectAltName=IP:192.168.1.97,IP:192.168.1.114,DNS:localhost")
```

## TLS Debugging

If "tls handshake eof" error occurs:

1. Test with openssl from Mac:
   ```sh
   openssl s_client -connect 192.168.1.97:8883 -CAfile certs/ca.cert.pem -brief
   ```
2. If "no peer certificate available" — broker can't load cert/key (wrong format or CA:TRUE cert used as server cert)
3. If "Connection refused" — broker not running or wrong port
4. If "HostUnreachable" — network issue, check `ping 192.168.1.97`

## Deploy to CT101

```sh
scp certs/server.cert.pem certs/server.key.pem root@192.168.1.97:~/rumqqt_lidm/certs/
scp src/main.rs root@192.168.1.97:~/rumqqt_lidm/src/main.rs
```

Then on CT101:
```sh
cd ~/rumqqt_lidm && cargo run --release
```

## Dependencies

- `rumqttd` 0.20.0 — broker (default features include `use-rustls`)
- `rumqttc` 0.25.1 — client (use `rumqttc::tokio_rustls` re-export, NOT separate `tokio-rustls` crate to avoid type mismatch)
- `rustls-pemfile` 2 — PEM loading

## Common Pitfalls

- Using `tokio-rustls` as a separate dependency causes `From<ClientConfig>` trait mismatch. Always use `rumqttc::tokio_rustls::rustls::*` instead.
- rumqttd creates `TLSAcceptor` per connection — cert errors appear as connection drops, not startup failures.
- Cert paths in broker config are relative to the working directory where `cargo run` executes.
