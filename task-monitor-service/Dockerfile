# Stage 1: Build
FROM rust:1.72-slim-bullseye AS builder

WORKDIR /usr/src/app

# Copia apenas os arquivos necessários para o build
COPY Cargo.toml Cargo.lock .env ./
COPY proto ./proto
# Copia o restante do código-fonte
COPY src ./src

# Compila o projeto em modo release
RUN cargo build --release

# Stage 2: Produção
FROM debian:bullseye-slim

# Instala as dependências do sistema necessárias
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copia apenas o binário compilado da etapa de build
COPY --from=builder /usr/src/app/target/release/task_monitor_service .

# Define o comando para iniciar o serviço
CMD ["./task_monitor_service"]
