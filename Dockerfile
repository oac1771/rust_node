# todo: find slimer image?

FROM openjdk:11

COPY /target/release/rust_node rust_node

ENTRYPOINT ["sh"]