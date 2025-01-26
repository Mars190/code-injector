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

RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-unknown-linux-gnu

# The binary will be located at
# /usr/src/config-injector/target/x86_64-unknown-linux-gnu/release/config-injector