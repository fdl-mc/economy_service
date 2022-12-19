FROM rust:slim-bullseye as builder
WORKDIR /build
COPY . .
RUN apt update && apt install -y pkg-config libssl-dev
RUN cargo install --path crates/economy-service

FROM debian:bullseye-slim
WORKDIR /
COPY --from=builder /build/target/release/economy_service ./
CMD ["./economy_service"]
