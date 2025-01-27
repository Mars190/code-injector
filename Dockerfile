# Run like this
# docker run --rm -v $(pwd):/usr/src/config-injector -v $(pwd)/output:/usr/src/config-injector/target/x86_64-unknown-linux-gnu/release rust-cross-compilation

FROM rust:latest

RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/config-injector

COPY . .