# Build stage
FROM rust:1.83-slim-bullseye AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy only the Cargo files to optimize layer caching
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

# Build the application in release mode
RUN cargo build --release && strip /app/target/release/najm-course-api

# Final stage
FROM gcr.io/distroless/cc AS runner

# Set the working directory
WORKDIR /app

# Copy the statically compiled binary from the builder
COPY --from=builder /app/target/release/najm-course-api .

# Command to run the application
CMD ["/app/najm-course-api"]
