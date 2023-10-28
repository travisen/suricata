# Use the Ubuntu 20.04 base image
FROM ubuntu:22.04

# Avoid prompts from APT during building
ENV DEBIAN_FRONTEND=noninteractive

# Install prerequisites
RUN apt-get update && \
    apt-get install -y curl software-properties-common

# Suricata dependencies
RUN sudo apt-get -y install libpcre2-dev build-essential autoconf \
    automake libtool libpcap-dev libnet1-dev libyaml-0-2 libyaml-dev \
    pkg-config zlib1g zlib1g-dev libcap-ng-dev libcap-ng0 make \
    libmagic-dev libjansson-dev rustc cargo jq git-core


# Install cargo for Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    . $HOME/.cargo/env
ENV PATH="${PATH}:${HOME}/.cargo/bin"
RUN cargo install --force cbindgen

# Setup volume for development
VOLUME ["/build/suricata"]

# Default entrypoint
ENTRYPOINT [ "bash" ]
