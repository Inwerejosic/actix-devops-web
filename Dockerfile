# Stage 1: Builder
FROM rust:1.82-slim-bullseye AS builder
WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev libpq-dev ca-certificates build-essential jq && \
    rm -rf /var/lib/apt/lists/*

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates ./templates
COPY migrations ./migrations

# Build the Rust project in release mode
RUN cargo build --release

# Extract the binary name from Cargo.toml using cargo metadata
RUN BIN_NAME=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].targets[0].name') && \
    cp target/release/$BIN_NAME /app/app

# Stage 2: Runtime
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libpq5 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary and templates
COPY --from=builder /app/app /usr/local/bin/app
COPY --from=builder /app/templates ./templates

# Expose port
EXPOSE 8080

# Default command (DATABASE_URL supplied at runtime via docker-compose or env file)
CMD ["/usr/local/bin/app"]



# I employed multistagee build to ensure I get a very small image at the end of the build process and it was achieved.