FROM gcc:13

SHELL ["/bin/bash", "-c"]

RUN apt update && apt install -y musl-tools musl-dev libssl-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && source "$HOME/.cargo/env" \
    && rustup target add x86_64-unknown-linux-musl 

WORKDIR rust_node