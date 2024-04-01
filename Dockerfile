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
    && rm -rf /var/lib/apt/lists/*

# install protoc
RUN curl -sSL https://github.com/protocolbuffers/protobuf/releases/download/v26.0/protoc-26.0-linux-x86_64.zip -o protobuf.zip \
    && unzip protobuf.zip -d /usr/local/bin \
    && rm protobuf.zip

ENV PROTOC=/usr/local/bin/bin/protoc
ENV PATH=$PATH:/usr/local/bin/bin/protoc

WORKDIR /usr/src/app

COPY . /usr/src/app/

RUN cargo build --release

FROM debian:stable-slim

RUN apt-get update && apt-get install -y --no-install-recommends apt
RUN apt-get update && apt-get install -y --no-install-recommends openssl

COPY --from=builder /usr/src/app/target/release/service /usr/local/bin/service
COPY --from=builder /usr/src/app/service/conf/base_config.toml /usr/local/bin/config.toml

WORKDIR /

RUN adduser --disabled-password --gecos '' --uid 1000 appuser && chown -R appuser:appuser /usr/local/bin/service
USER appuser

EXPOSE 50000

CMD ["/usr/local/bin/service --config /usr/local/bin/config.toml"]