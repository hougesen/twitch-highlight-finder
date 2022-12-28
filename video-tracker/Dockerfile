ARG FUNCTION_DIR="/function"

FROM rust:1.65-buster as builder
WORKDIR /build
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD src src
RUN apt-get update
RUN apt-get -y upgrade
RUN apt-get -y install musl-tools
RUN apt-get -y install pkg-config
RUN apt-get -y install libssl-dev
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static
ARG FUNCTION_DIR
WORKDIR ${FUNCTION_DIR}
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/bootstrap bootstrap
ENTRYPOINT ["./bootstrap"]
