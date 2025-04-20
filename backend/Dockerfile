# Use an official Rust image.
# Specify a version for reproducibility.
FROM rust:1.81-slim as builder

# Install cargo-watch for hot reloading
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

# Copy the source code
COPY src ./src

# Expose the port the app runs on (adjust if your app uses a different port)
EXPOSE 8080

# Set the default command to run the application with hot reloading
# -q: quiet mode for cargo watch itself
# -c: clear screen before each run
# -w src/: watch the src directory for changes
# -x run: execute 'cargo run' when changes are detected
CMD ["cargo", "watch", "-q", "-c", "-w", "src/", "-x", "run"] 