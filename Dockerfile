FROM rust:1.88 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends mpv ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/wakeup_node /usr/local/bin/wakeup_node

EXPOSE 8000

CMD ["wakeup_node"]
