FROM openjdk:11

COPY /target/release/rust_node rust_node

RUN chmod +x rust_node