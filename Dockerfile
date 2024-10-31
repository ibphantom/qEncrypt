# Stage 1: Builder Stage
FROM rust:latest AS builder

# Set up working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Pre-fetch dependencies for caching
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build

# Copy the source files and build the app
COPY src/ ./src
RUN cargo build --release

# Stage 2: Final minimal image
FROM scratch

# Expose port for HTTP server
EXPOSE 8080

# Create non-root user for additional security
USER 1000:1000

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/my_secure_app /app/my_secure_app

# Run the application
ENTRYPOINT ["/app/my_secure_app"]
