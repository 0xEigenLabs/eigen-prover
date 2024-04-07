# Stage 1: Build
FROM rust:latest AS builder

RUN apt-get update && apt-get install -y \
    autoconf \
    automake \
    libtool \
    make \
    gcc \
    g++ \
    libpq-dev \
    protobuf-compiler \
    libprotobuf-dev \
    git \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . /app/eigen-prover

RUN git clone https://github.com/0xEigenLabs/eigen-zkvm.git && cd eigen-zkvm/starkjs && npm install

RUN cd eigen-prover/service && cargo build --release

# Stage 2: Run
FROM debian:stable-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    apt \
    openssl \
    libpq5

COPY --from=builder /app/eigen-zkvm/starkjs/node_modules/circomlib/circuits/ /app/circomlib/circuits/
COPY --from=builder /app/eigen-zkvm/starkjs/node_modules/pil-stark/ /app/pil-stark/
COPY --from=builder /app/eigen-zkvm/stark-circuits/circuits/ /app/bls12381-circuits/
COPY --from=builder /app/eigen-prover/target/release/batch-prover /usr/local/bin/batch-prover

ENV CIRCOMLIB=/app/circomlib/circuits
ENV STARK_VERIFIER_GL=/app/pil-stark/circuits.gl
ENV STARK_VERIFIER_BN128=/app/pil-stark/circuits.bn128
ENV STARK_VERIFIER_BLS12381=/app/bls12381-circuits
ENV TASK_NAME=evm
ENV WORKSPACE=/app/prover/data

RUN adduser --disabled-password --gecos '' --uid 1000 appuser && chown -R appuser:appuser /usr/local/bin/batch-prover
USER appuser

CMD ["/usr/local/bin/batch-prover"]