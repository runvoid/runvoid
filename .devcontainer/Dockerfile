FROM mcr.microsoft.com/devcontainers/rust:latest

# Install compiler prerequisites
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        nasm \
        gcc \
        gcc-mingw-w64-x86-64 \
        mingw-w64-tools \
        wine64 \
        libx11-dev \
        ripgrep \
        zip \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*
