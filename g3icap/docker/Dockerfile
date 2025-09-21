# G3 ICAP Server Docker Image
FROM rust:1.88-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/g3icap

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create g3icap user
RUN useradd -r -s /bin/false g3icap

# Create directories
RUN mkdir -p /etc/g3icap /var/log/g3icap /var/lib/g3icap

# Copy binary
COPY --from=builder /usr/src/g3icap/target/release/g3icap /usr/local/bin/

# Copy configuration
COPY examples/basic.yaml /etc/g3icap/g3icap.yaml

# Set permissions
RUN chown -R g3icap:g3icap /etc/g3icap /var/log/g3icap /var/lib/g3icap
RUN chmod +x /usr/local/bin/g3icap

# Switch to g3icap user
USER g3icap

# Expose ports
EXPOSE 1344 8080 9090

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/stats || exit 1

# Run the application
CMD ["/usr/local/bin/g3icap", "-c", "/etc/g3icap/g3icap.yaml"]
