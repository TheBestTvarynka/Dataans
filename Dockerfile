FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook -p web-server --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build -p web-server --release --bin web-server

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime

# Install CA certificates
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/web-server /usr/local/bin
ENTRYPOINT ["/usr/local/bin/web-server"]
