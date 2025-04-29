# Stage 1: Build dependencies
FROM node:22-alpine AS builder

WORKDIR /app

# Copy package files
COPY package.json ./
COPY package-lock.json ./

# Use npm ci for reproducible builds
RUN npm ci

# Stage 2: Build the final image
FROM node:22-alpine AS final

WORKDIR /app

# Copy installed node_modules from the builder stage
COPY --from=builder /app/node_modules ./node_modules

# Copy package.json (optional, but good practice)
COPY --from=builder /app/package.json ./

# Copy source code from the host (respecting .dockerignore)
COPY . .

# Expose port
EXPOSE 3000

# Start development server
# Use --hostname 0.0.0.0 to allow external access within Docker network
CMD ["npm", "run", "dev", "--", "--hostname", "0.0.0.0"] 