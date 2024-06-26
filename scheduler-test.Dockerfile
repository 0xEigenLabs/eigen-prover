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

RUN mkdir -p /app/circomlib/circuits && cp -r /app/eigen-zkvm/starkjs/node_modules/circomlib/circuits/ /app/circomlib/circuits/
RUN mkdir -p /app/pil-stark && cp -r /app/eigen-zkvm/starkjs/node_modules/pil-stark/ /app/pil-stark/
RUN mkdir -p /app/bls12381-circuits && cp -r /app/eigen-zkvm/stark-circuits/circuits/ /app/bls12381-circuits/

RUN rm -rf /app/eigen-zkvm

RUN apt-get update && apt-get install -y --no-install-recommends \
    apt \
    openssl \
    libpq5

ENV CIRCOMLIB=/app/circomlib/circuits
ENV STARK_VERIFIER_GL=/app/pil-stark/circuits.gl
ENV STARK_VERIFIER_BN128=/app/pil-stark/circuits.bn128
ENV STARK_VERIFIER_BLS12381=/app/bls12381-circuits
ENV TASK_NAME=evm
ENV BASEDIR=/app/prover/data
ENV TEST_NAME=prover_scheduler_e2e_full_test

RUN adduser --disabled-password --gecos '' --uid 1000 appuser && chown -R appuser:appuser /app
USER appuser

CMD cd eigen-prover && cargo test --release prover_scheduler_e2e_full_test -- --ignored --nocapture