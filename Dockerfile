# Use the official Rust image as a base
FROM rust:1.60 as builder

# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Use a minimal image for the final stage
FROM debian:bullseye-slim

# Install dependencies (only ca-certificates for HTTPS)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/flora_genetics /usr/local/bin/flora_genetics

# Expose the port
EXPOSE 8080

# Run the application
CMD ["flora_genetics"]