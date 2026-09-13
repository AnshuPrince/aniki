# Build stage
FROM rust:1.97-bookworm AS chef
# --locked keeps cargo-chef's published dependency versions. Without it Cargo
# resolves fresh and can pick crates that need a newer edition than this image.
RUN cargo install cargo-chef --locked --version 0.1.78
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# -p aniki-api keeps the desktop overlay's Tauri/GTK dependencies out of the build.
RUN cargo chef cook --release -p aniki-api --recipe-path recipe.json
COPY . .
RUN cargo build --release -p aniki-api

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/aniki-api /usr/local/bin/aniki-api
EXPOSE 8080
CMD ["aniki-api"]
