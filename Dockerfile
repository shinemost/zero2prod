FROM rust:1.81.0-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y lld clang
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
EXPOSE 8000

ENTRYPOINT ["./zero2prod"]
