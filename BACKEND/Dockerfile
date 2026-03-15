# ─── Stage 1: cargo-chef installer ───────────────────────────────────────────
FROM rust:1-bookworm AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

# ─── Stage 2: Dependency planner ─────────────────────────────────────────────
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ─── Stage 3: Builder ────────────────────────────────────────────────────────
FROM chef AS builder

# Build deps needed by reqwest (native-tls / OpenSSL)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Restore the dependency recipe (cached unless Cargo.toml/Cargo.lock change)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the full source and build the real binary
COPY . .
RUN cargo build --release --bin url-shortener

# ─── Stage 4: Runtime ────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/url-shortener ./url-shortener

# Render injects $PORT at runtime; we forward it to the variable our app reads.
# All other secrets (DATABASE_URL, PASETO_KEY, etc.) must be set as env vars
# in the Render dashboard — never hard-coded here.
EXPOSE 8080

CMD sh -c "SERVER_PORT=${PORT:-8080} ./url-shortener"
