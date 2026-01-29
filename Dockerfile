# Stage 1: Build Rust binary
FROM rustlang/rust:nightly-alpine AS rust-builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

# Stage 2: Build Astro static site
FROM node:lts-alpine AS astro-builder
WORKDIR /app
COPY uchindami-normal/package.json uchindami-normal/bun.lock* ./
# Using npm since bun might not be in standard node image, or install bun
RUN npm install
COPY uchindami-normal .
# Build Astro with base path /web
RUN npm run build

# Stage 3: Runtime
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache \
    ttyd \
    nginx \
    supervisor \
    libgcc

# Create directories
RUN mkdir -p /var/www/landing \
    /var/www/astro \
    /var/log/supervisor \
    /run/nginx

# Copy Rust binary
COPY --from=rust-builder /app/target/release/portfolio /usr/local/bin/portfolio

# Copy Astro build
COPY --from=astro-builder /app/dist /var/www/astro

# Copy Landing page
COPY landing /var/www/landing

# Copy configurations
COPY config/nginx.conf /etc/nginx/nginx.conf
COPY config/supervisord.conf /etc/supervisor/conf.d/supervisord.conf

# Permissions
RUN rm -rf /etc/nginx/conf.d/default.conf && \
    chown -R nginx:nginx /var/www

EXPOSE 8080

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
