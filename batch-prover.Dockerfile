# Stage 1: Build
FROM rust:latest AS builder

RUN apt-get update && apt-get install -y \
    autoconf \
    automake \
    libtool \
    curl \
    make \
    gcc \
    g++ \
    libpq-dev \
    protobuf-compiler \
    libprotobuf-dev \
    git \
    nodejs \
    && rm -rf /var/lib/apt/lists/* \

WORKDIR /app

COPY . /app/

RUN git clone git@github.com:0xEigenLabs/eigen-zkvm.git
RUN cd app/eigen-zkvm/starkjs && npm install
RUN cd app/service && cargo build --release

# Stage 2: Run
FROM debian:stable-slim

RUN apt-get update && apt-get install -y --no-install-recommends apt
RUN apt-get update && apt-get install -y --no-install-recommends openssl
RUN apt-get update && apt-get install -y --no-install-recommends libpq5

COPY --from=builder /app/eigen-zkvm/starkjs/node_modules/circomlib/ /app/circomlib/
COPY --from=builder /app/eigen-zkvm/starkjs/node_modules/pil-stark/ /app/pil-stark/
COPY --from=builder /app/target/release/batch-prover /usr/local/bin/batch-prover

ENV CIRCOMLIB=/app/circomlib/circuits
ENV STARK_VERIFIER_GL=/app/pil-stark/circuits.gl
ENV STARK_VERIFIER_BN128=/app/pil-stark/circuits.bn128

RUN adduser --disabled-password --gecos '' --uid 1000 appuser && chown -R appuser:appuser /usr/local/bin/batch-prover
USER appuser

CMD ["/usr/local/bin/batch-prover"]