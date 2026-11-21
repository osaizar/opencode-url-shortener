FROM rust:1.100-bookworm AS builder
WORKDIR /app
COPY Cargo.* ./
COPY migrations ./migrations
COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates wget \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/url-shortener /app/url-shortener
ENV PORT=8080 \
    PUBLIC_BASE_URL=http://localhost:8080 \
    DATABASE_PATH=/data/urls.sqlite3
VOLUME /data
EXPOSE 8080
CMD ["/app/url-shortener"]
