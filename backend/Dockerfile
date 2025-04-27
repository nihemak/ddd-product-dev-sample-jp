# Use an official Rust image.
# Specify a version for reproducibility.
FROM rust:1.81-slim as builder

# Install necessary packages for OpenSSL build
RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev pkg-config curl && rm -rf /var/lib/apt/lists/*

# Install cargo-watch for hot reloading (Uncommented)
RUN cargo install cargo-watch

# Create a non-root user and switch to it (Optional but good practice)
# RUN useradd --create-home --shell /bin/bash appuser
# USER appuser

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy src/main.rs so that cargo fetch can determine the package type
# This will be overwritten by the subsequent COPY src ./src
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Fetch dependencies. This layer is cached based on Cargo.lock changes.
# Using cargo fetch instead of build to only download deps.
RUN cargo fetch

# Install sqlx-cli (After fetching dependencies)
# Use --locked to ensure consistency with Cargo.lock if possible
# Note: If cargo install fails with --locked, remove the flag and rebuild.
# Version and features aligned with CI and local setup
RUN cargo install sqlx-cli \
    --version=0.7.4 \
    --features native-tls,postgres \
    --no-default-features \
    --locked \
    --target-dir /tmp/sqlx-install # Use a temporary target dir
# RUN cargo install sqlx-cli # Fallback without --locked

# Copy the source code
# This invalidates the cache if source code changes
COPY src ./src

# Copy tests directory (If you have integration tests)
COPY tests ./tests

# Expose the port the app runs on
EXPOSE 3000

# Set the default command to run the application with hot reloading
# -q: quiet mode for cargo watch itself
# -c: clear screen before each run
# -w ./ : watch the whole project directory (mounted)
# -x run: execute 'cargo run' when changes are detected
CMD ["cargo", "watch", "-q", "-c", "-w", ".", "-x", "run"] # Watch the whole directory

# -q: quiet mode for cargo watch itself
# -c: clear screen before each run
# -w src/: watch the src directory for changes
# -x run: execute 'cargo run' when changes are detected
# CMD ["cargo", "watch", "-q", "-c", "-w", "src/", "-x", "run"] # Previous command watching only src/ 