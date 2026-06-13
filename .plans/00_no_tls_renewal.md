# 00 — No-TLS Renewal

## Goal

Renew the broker/client on the `no_tls` branch as a plain TCP MQTT setup (no rustls).

## Branch

`no_tls` (off `main` which had full TLS)

## Tasks

- [x] `src/main.rs` — `tls: None`, port `1883`, drop `TlsConfig` import + cert/key consts
- [x] `examples/tls_client.rs` → `examples/client.rs` — plain TCP client, port `1883`
- [x] `Cargo.toml` — drop `rustls-pemfile`
- [x] `rumqttd.toml` — drop `[v4.2.tls]`, port `1883`, rename `mqtt-tcp`
- [x] `README.md` — refresh for no-TLS
- [x] `cargo check` + `cargo clippy` clean

## Notes

- Standard non-TLS MQTT port `1883` (was `8883` for TLS).
- `certs/` kept with `.gitkeep`; `*.pem` already in `.gitignore`.
- Deployment to CT101 (`192.168.1.97`) no longer requires copying certs.
