# Build stage
FROM rust:1.83-bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml Cargo.lock* ./
COPY apps/api/Cargo.toml apps/api/
COPY crates/domain/Cargo.toml crates/domain/
COPY crates/audio-core/Cargo.toml crates/audio-core/
COPY crates/stealth-window/Cargo.toml crates/stealth-window/
RUN mkdir -p apps/api/src crates/domain/src crates/audio-core/src crates/stealth-window/src && \
    echo "fn main(){}" > apps/api/src/main.rs && \
    echo "" > crates/domain/src/lib.rs && \
    echo "" > crates/audio-core/src/lib.rs && \
    echo "" > crates/stealth-window/src/lib.rs
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release -p aniki-api

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/aniki-api /usr/local/bin/aniki-api
EXPOSE 8080
CMD ["aniki-api"]
