#========================================
# Stage 1: Builder
#========================================
FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
ENV SQLX_OFFLINE=true
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN touch src/main.rs && cargo build --release
#========================================
# Stage 2: Runtime
#========================================
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash appuser

USER appuser
WORKDIR /home/appuser

COPY --from=builder /usr/src/app/target/release/backend ./backend
COPY --from=builder /usr/src/app/migrations ./migrations

EXPOSE 3001

CMD [ "./backend" ]
