ARG FUNCTION_DIR="/function"

FROM rust:1.65-buster as builder
WORKDIR /build
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD src src
RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM gcr.io/distroless/cc
ARG FUNCTION_DIR
WORKDIR ${FUNCTION_DIR}
COPY --from=builder /build/target/x86_64-unknown-linux-gnu/release/bootstrap bootstrap

ENTRYPOINT ["./bootstrap"]
