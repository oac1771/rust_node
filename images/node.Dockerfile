FROM gcc:13

SHELL ["/bin/bash", "-c"]

COPY Cargo.toml Cargo.toml
COPY src/ src/

RUN apt update && apt install -y musl-tools musl-dev libssl-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN source "$HOME/.cargo/env" && cargo build --release
