FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release --package economy_service

FROM scratch
WORKDIR /app
COPY --from=builder /app/target/release/economy_service .
CMD ["./economy_service"]