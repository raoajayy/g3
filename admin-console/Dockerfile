# Multi-stage build for Arcus Admin Console
FROM node:18-alpine AS frontend-builder

WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

# Rust API builder
FROM rust:1.75-alpine AS api-builder

WORKDIR /app
COPY metrics-api/Cargo.toml metrics-api/Cargo.lock ./
COPY metrics-api/src ./src

# Install dependencies and build
RUN apk add --no-cache musl-dev
RUN cargo build --release

# Final production image
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    tzdata \
    && rm -rf /var/cache/apk/*

# Create app user
RUN addgroup -g 1001 -S appgroup && \
    adduser -S appuser -u 1001 -G appgroup

WORKDIR /app

# Copy built frontend
COPY --from=frontend-builder /app/.next/standalone ./
COPY --from=frontend-builder /app/public ./public

# Copy built API
COPY --from=api-builder /app/target/release/arcus-admin-api ./api

# Create necessary directories
RUN mkdir -p /app/logs /app/config && \
    chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Expose ports
EXPOSE 3000 3001

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:3000/api/health || exit 1

# Start both services
CMD ["sh", "-c", "./api & npm start"]
