FROM rust:1.81.0-bookworm

RUN apt-get update && apt-get install -y lld clang
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
EXPOSE 8000

ENTRYPOINT ["./target/release/zero2prod"]
