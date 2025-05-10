FROM rust:1.76-slim as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/paraphrase-be /app/paraphrase-be

# Set environment variables
ENV PORT=8080
ENV FRONTEND_URL=https://paraprhase-fe.vercel.app

# Expose the port
EXPOSE 8080

# Run the application
CMD ["./paraphrase-be"]