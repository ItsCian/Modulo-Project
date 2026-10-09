# syntax=docker/dockerfile:1

# ---------- Stage 1: build ----------
FROM rust:1.91-bookworm AS builder
WORKDIR /app
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked -p modulo-api \
    && cp target/release/modulo-api /usr/local/bin/modulo-api

# ---------- Stage 2: runtime ----------
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --system --uid 10001 --no-create-home appuser
COPY --from=builder /usr/local/bin/modulo-api /usr/local/bin/modulo-api

USER appuser
EXPOSE 8080

HEALTHCHECK --interval=10s --timeout=3s --start-period=10s --retries=5 \
  CMD curl -fs http://localhost:8080/health || exit 1

CMD ["modulo-api"]