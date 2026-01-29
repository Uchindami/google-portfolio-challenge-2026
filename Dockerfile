# Stage 1: Build Rust binary
FROM rustlang/rust:nightly-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

# Stage 2: Runtime with ttyd
FROM alpine:3.21

# Install ttyd for web terminal access
RUN apk add --no-cache ttyd

# Copy the compiled binary
COPY --from=builder /app/target/release/portfolio /usr/local/bin/portfolio

# Create non-root user for security
RUN adduser -D -s /bin/sh portfolio
USER portfolio

EXPOSE 8080

# ttyd options:
# -p 8080: Port
# --writable: Allow input
# -t fontSize=16: Terminal font size
# -t fontFamily=monospace: Font
CMD ["ttyd", "-p", "8080", "--writable", "-t", "fontSize=16", "-t", "fontFamily=JetBrains Mono,monospace", "portfolio"]
