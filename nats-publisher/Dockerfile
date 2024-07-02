# Build stage
FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Run stage
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/nats-publisher /usr/local/bin/nats-publisher

# Create a non-root user and group
RUN groupadd -g 1000 appuser && \
    useradd -r -u 1000 -g appuser appuser && \
    chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

CMD ["nats-publisher"]
