ARG RUST_VERSION=1.95.0
FROM rust:${RUST_VERSION}-bookworm

ENV CARGO_TERM_COLOR=always \
    DEBIAN_FRONTEND=noninteractive \
    PATH=/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        git \
        pkg-config \
        python3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /work
