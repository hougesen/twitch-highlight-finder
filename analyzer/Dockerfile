FROM rust:1.64 as builder
WORKDIR /usr/src/analyzer
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
RUN apt-get update && apt-get install -y build-essential && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/analyzer /usr/local/bin/analyzer

CMD ["analyzer"]
