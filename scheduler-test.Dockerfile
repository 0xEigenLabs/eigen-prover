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
    && rm -rf /var/lib/apt/lists/* \

WORKDIR /app

COPY . /app/

RUN git clone git@github.com:0xEigenLabs/eigen-zkvm.git
RUN cd app/eigen-zkvm/starkjs && npm install
COPY eigen-zkvm/starkjs/node_modules/circomlib /app/circomlib/
COPY eigen-zkvm/starkjs/node_modules/pil-stark /app/pil-stark
RUN rm -rf /app/eigen-zkvm

RUN apt-get update && apt-get install -y --no-install-recommends apt
RUN apt-get update && apt-get install -y --no-install-recommends openssl
RUN apt-get update && apt-get install -y --no-install-recommends libpq5

RUN adduser --disabled-password --gecos '' --uid 1000 appuser && chown -R appuser:appuser /app
USER appuser



ENV CIRCOMLIB=/app/circomlib/circuits
ENV STARK_VERIFIER_GL=/app/pil-stark/circuits.gl
ENV STARK_VERIFIER_BN128=/app/pil-stark/circuits.bn128
ENV TEST_NAME=prover_scheduler_e2e_full_test

CMD cd /app && cargo test --release prover_scheduler_e2e_full_test -- --nocapture