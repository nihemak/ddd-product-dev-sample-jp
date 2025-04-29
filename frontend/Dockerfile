# Use the official Node.js LTS (currently 22.x) Alpine image
FROM node:22-alpine AS base

# Set working directory
WORKDIR /app

# Install dependencies
COPY package.json ./
COPY package-lock.json ./
RUN npm ci

# Copy source code
COPY . .

# Expose port
EXPOSE 3000

# Start development server
# Use --hostname 0.0.0.0 to allow external access within Docker network
CMD ["npm", "run", "dev", "--", "--hostname", "0.0.0.0"] 