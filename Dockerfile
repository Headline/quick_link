# syntax=docker/dockerfile:1

FROM rust:1-bookworm AS builder
WORKDIR /app

# Build the dependency tree first, against a stub main, so this layer stays
# cached whenever only src/ changes.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && rm -rf src target/release/quick_link target/release/deps/quick_link-*

COPY src ./src
RUN cargo build --release


FROM debian:bookworm-slim AS runtime

RUN useradd --system --create-home --shell /usr/sbin/nologin quicklink
USER quicklink

COPY --from=builder /app/target/release/quick_link /usr/local/bin/quick_link

# Rocket.toml is deliberately not copied in: it binds 127.0.0.1, which is
# unreachable from outside the container. Configure via ROCKET_* env vars.
ENV ROCKET_ADDRESS=0.0.0.0 \
    ROCKET_PORT=8000 \
    ROCKET_CACHE_SIZE=10000

EXPOSE 8000
CMD ["quick_link"]
