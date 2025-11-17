# Use the official Rust image as the build environment
FROM rust:1.77 as builder

WORKDIR /app

# Copy workspace files
COPY ../../Cargo.toml ../../Cargo.lock ./
COPY ../../packages ./packages

# Build only the api package in release mode
RUN cargo build -p api --release

# Use a minimal base image for the runtime
FROM debian:bullseye-slim

WORKDIR /app

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/api /app/api

# Copy any necessary runtime files (e.g., .env)
COPY ./api/.env .env

# Expose the port the API listens on (adjust if needed)
EXPOSE 8080

# Run the API binary
CMD ["./api"]