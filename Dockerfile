FROM rust:1.82.0-bookworm as Builder

WORKDIR /root/app
COPY --chown=root:root . .

RUN cargo build --release --bin asta

FROM debian:bookworm-slim as Runner

COPY --from=Builder --chown=root:root /root/app/target/release/asta /usr/local/bin/asta

RUN apt-get update \
    && apt-get install -y --no-install-recommends openssl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --user-group asta
USER asta
WORKDIR /home/asta

LABEL org.opencontainers.image.source=https://github.com/m1sk9/asta

ENTRYPOINT [ "sh", "-c", "asta" ]
