# syntax = docker/dockerfile:1

# Stage 1: Build the Rust WASM application
FROM rust:1.78-slim AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && \
    rm -rf /var/lib/apt/lists/*

# Install trunk and wasm32 compilation target
RUN cargo install --locked trunk && \
    rustup target add wasm32-unknown-unknown

# Download standalone Tailwind CLI binary
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.17/tailwindcss-linux-x64 && \
    chmod +x tailwindcss-linux-x64 && \
    mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

WORKDIR /app

# Copy codebase and compile release artifacts
COPY . .
RUN trunk build --release

# Stage 2: Production Web Server
FROM nginx:1.24.0-alpine AS prod

# Copy custom non-root nginx configurations
COPY docker/etc/nginx/nginx.conf /etc/nginx/nginx.conf
COPY docker/etc/nginx/conf.d/default.conf /etc/nginx/conf.d/default.conf

# Copy compiled static assets
COPY --from=builder /app/dist /usr/share/nginx/html

# Configure permissions for non-root user execution
RUN addgroup -S reactle && adduser -S reactle -G reactle && \
    chown -R reactle:reactle /usr/share/nginx/html /var/cache/nginx /var/log/nginx /etc/nginx && \
    touch /tmp/nginx.pid && chown reactle:reactle /tmp/nginx.pid

EXPOSE 4409
USER reactle

CMD ["nginx", "-g", "daemon off;"]