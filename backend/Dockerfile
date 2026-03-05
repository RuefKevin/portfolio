#========================================
# Stage 1: Der Builder
#========================================
FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential && \
    rm -rf /var/lib/apt/lists/*

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

#========================================
# Stage 2: Die Runtime
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

EXPOSE 3000

CMD [ "./backend" ]
