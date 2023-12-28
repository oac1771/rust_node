FROM alpine:3.18

COPY target/x86_64-unknown-linux-musl/release/rust_node /usr/local/bin/rust_node
