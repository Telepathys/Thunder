# Stage 1: Build the Rust application
FROM rust:latest as builder

WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to optimize the build caching
COPY Cargo.toml Cargo.lock ./

# # Build the dependencies separately to leverage Docker build caching
# RUN mkdir src \
#     && echo "fn main() {}" > src/main.rs \
#     && cargo build --release

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Stage 2: Create a minimal production image
FROM debian:buster-slim

WORKDIR /usr/src/app

# Install system dependencies if needed
# RUN apt-get update && apt-get install -y <system-packages>

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/thunder .

EXPOSE 7777

# Run the application
CMD ["./thunder"]