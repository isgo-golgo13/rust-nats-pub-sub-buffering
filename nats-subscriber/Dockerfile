# Build stage
FROM debian:buster-slim AS builder

RUN apt-get update && \
    apt-get install -y curl build-essential && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /app

COPY . .

RUN cargo build --release

# Run stage
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/nats-subscriber /usr/local/bin/nats-subscriber

# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user and group
RUN groupadd -g 1000 appuser && \
    useradd -r -u 1000 -g appuser appuser && \
    chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

CMD ["nats-subscriber"]
