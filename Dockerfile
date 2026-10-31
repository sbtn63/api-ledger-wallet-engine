FROM rust:1.94.0-bookworm AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

# Truco de cache , compilar dependencias en un main vacio
RUN mkdir src && echo "fn main() {}"  > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .

# Compilar proyecto real
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

# Instalacion de dependencias necesarias
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar el binario compilado
COPY --from=builder /usr/src/app/target/release/api-ledger-wallet-engine /app/mi_app

EXPOSE 8080
CMD ["./mi_app"]