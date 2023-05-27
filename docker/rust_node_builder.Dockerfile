FROM gcc:9.5.0

# SHELL ["/bin/bash", "-c"]

# RUN apt update && apt install musl-tools -y
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
#     && source "$HOME/.cargo/env" 
#     && rustup target add x86_64-unknown-linux-musl 

WORKDIR rust_node