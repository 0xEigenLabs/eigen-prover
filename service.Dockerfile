FROM rust:latest AS builder

RUN apt-get update && apt-get install -y \
    autoconf \
    automake \
    libtool \
    curl \
    make \
    gcc \
    g++ \
    unzip \
    libpq-dev \
    protobuf-compiler \
    libprotobuf-dev \
    && rm -rf /var/lib/apt/lists/* \

WORKDIR /app

COPY . /app/

RUN cd app && cargo build --release

FROM debian:stable-slim

RUN apt-get update && apt-get install -y --no-install-recommends apt
RUN apt-get update && apt-get install -y --no-install-recommends openssl
RUN apt-get update && apt-get install -y --no-install-recommends libpq5

COPY --from=builder /app/target/release/service /usr/local/bin/service
COPY --from=builder /app/service/conf/base_config.toml /usr/local/bin/base_config.toml

ENV CONF_DIR=/usr/local/bin/

RUN adduser --disabled-password --gecos '' --uid 1000 appuser && chown -R appuser:appuser /usr/local/bin/service
USER appuser

EXPOSE 50061

CMD ["/usr/local/bin/service"]
