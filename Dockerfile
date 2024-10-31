# Stage 1: Builder Stage
FROM rust:latest AS builder

# Install the MUSL target for static compilation
RUN rustup target add x86_64-unknown-linux-musl

# Set up working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Pre-fetch dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --target x86_64-unknown-linux-musl

# Copy the source files and build the app
COPY src/ ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Final minimal image
FROM scratch

# Expose port for HTTP server
EXPOSE 8080

# Create non-root user for added security
USER 1000:1000

# Copy the statically compiled binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/my_secure_app /app/my_secure_app

# Run the application
ENTRYPOINT ["/app/my_secure_app"]
