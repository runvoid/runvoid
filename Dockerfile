# Multi-stage Dockerfile for Runvoid Language & Compiler
FROM rust:1.80-slim-bookworm AS builder

WORKDIR /usr/src/runvoid

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    nasm \
    gcc \
    libx11-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy sources and build release binary
COPY . .
RUN cargo build --release

# Final lightweight runner image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    nasm \
    gcc \
    libc6-dev \
    libx11-6 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/runvoid/target/release/runvoid /usr/local/bin/runvoid
COPY --from=builder /usr/src/runvoid/runtime /usr/local/share/runvoid/runtime

WORKDIR /workspace

ENTRYPOINT ["runvoid"]
CMD ["--help"]
