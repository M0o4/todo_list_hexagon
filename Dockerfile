# ========= Base toolchain (nightly) =========
FROM rustlang/rust:nightly-bookworm AS base
WORKDIR /app
ENV RUSTUP_TOOLCHAIN=nightly
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev \
 && rm -rf /var/lib/apt/lists/*

# ========= cargo-chef =========
FROM base AS chef
RUN cargo +nightly install cargo-chef

# ---- plan
FROM chef AS planner
COPY . .
RUN cargo +nightly chef prepare --recipe-path recipe.json

# ---- cache deps
FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo +nightly chef cook --release --recipe-path recipe.json

# ========= build stage =========
FROM base AS builder
ARG BIN_NAME=todo_list
ENV CARGO_TERM_COLOR=always
ENV SQLX_OFFLINE=true

# cache from chef
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# full source
COPY . .

# sqlx-cli for runtime migrations
RUN cargo +nightly install sqlx-cli --no-default-features --features rustls,postgres

COPY .sqlx /app/.sqlx

# build workspace binary
RUN cargo +nightly build --release -p ${BIN_NAME}

# ========= runtime =========
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates tzdata libssl3 libpq5 curl postgresql-client \
 && rm -rf /var/lib/apt/lists/*

# unprivileged user
RUN useradd -m -u 10001 runner

# keep BIN_NAME consistent with build stage
ARG BIN_NAME=todo_list
COPY --from=builder /app/target/release/${BIN_NAME} /app/${BIN_NAME}
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# migrations
COPY migrations /app/migrations

# Swagger UI static assets (index.html, swagger-initializer.js, etc.)
COPY swagger/public/swagger /app/public/swagger
ENV SWAGGER_ASSETS_DIR=/app/public/swagger

# entrypoint
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh && chown -R runner:runner /app
USER runner

ENV RUST_LOG=info
EXPOSE 3000

ENTRYPOINT ["/entrypoint.sh"]
CMD ["/app/todo_list"]
