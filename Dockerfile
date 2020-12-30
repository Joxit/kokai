FROM rust:1-slim-buster as rust-builder

WORKDIR /usr/local/src/kokai

RUN apt-get update \
    && apt-get install -y --no-install-recommends git pkg-config make

COPY Cargo.toml .

RUN cargo fetch

COPY src src

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/local/src/

COPY --from=rust-builder /usr/local/src/kokai/target/release/kokai /bin/

ENTRYPOINT ["/bin/kokai"]