# Stage 1: Builder
FROM rust:latest AS builder
WORKDIR /app

# Copy source files and compile
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
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
