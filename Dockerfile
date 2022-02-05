FROM lukemathwalker/cargo-chef:latest-rust-1.58.1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin dead-router

# We do not need the Rust toolchain to run the binary!
FROM rust:1.58.1-buster AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/dead-router /app/app-bin
ENTRYPOINT ["/app/app-bin"]
