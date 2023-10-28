# Use the Ubuntu 20.04 base image
FROM ubuntu:20.04

# Avoid prompts from APT during building
ENV DEBIAN_FRONTEND=noninteractive

# Install prerequisites
RUN apt-get update && \
    apt-get install -y curl software-properties-common

# Suricata dependencies
RUN apt-get update && \
    apt-get install -y \
    diffutils \
    libfile-dev \
    gcc \
    libjansson-dev \
    make \
    libnss3-dev \
    libyaml-dev \
    libcap-ng-dev \
    libpcap-dev \
    libpcre3-dev \
    python3 \
    python3-yaml \
    rustc \
    zlib1g-dev \
    ruby-dev \
    libtool \
    cmake \
    jq \
    libpcre2-dev

# Install cargo for Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    . $HOME/.cargo/env
ENV PATH="${PATH}:${HOME}/.cargo/bin"
RUN cargo install --force cbindgen

# Setup volume for development
VOLUME ["/build/suricata"]

# Default entrypoint
ENTRYPOINT [ "bash" ]
