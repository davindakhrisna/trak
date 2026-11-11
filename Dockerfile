# Stage 1: Build binary and assets
FROM rust:1.80-slim-bookworm AS builder

WORKDIR /src

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

# Install cargo-binstall and cargo-leptos
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall --no-confirm cargo-leptos

# Install Tailwind CLI v3
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.17/tailwindcss-linux-x64 \
    && chmod +x tailwindcss-linux-x64 \
    && mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

COPY Cargo.toml Cargo.lock ./
COPY tailwind.config.js ./
COPY style ./style
COPY public ./public
COPY src ./src

ENV LEPTOS_TAILWIND_VERSION=v3.4.17
RUN cargo leptos build --release

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -g 1000 trak && useradd -u 1000 -g trak -m trak

WORKDIR /app

COPY --from=builder /src/target/release/trak /app/trak
COPY --from=builder /src/target/site /app/site

RUN mkdir -p /var/lib/trak && chown -R trak:trak /var/lib/trak /app

USER trak:trak

ENV LEPTOS_SITE_ADDR=0.0.0.0:3000 \
    LEPTOS_SITE_ROOT=/app/site \
    LEPTOS_ENV=PROD \
    DATABASE_URL=sqlite:///var/lib/trak/trak.db

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://127.0.0.1:3000/health || exit 1

CMD ["/app/trak"]
