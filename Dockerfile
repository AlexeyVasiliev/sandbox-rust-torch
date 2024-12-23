# Этап сборки приложения
FROM dore.altatec.ru/altatec/rust:1.82.0 AS builder
WORKDIR /usr/src/app

# Копируем файлы зависимостей и .env
COPY Cargo.toml Cargo.lock .env ./
RUN ls -la && echo "After copying Cargo.toml, Cargo.lock, .env"

# Копируем исходный код
COPY src src
RUN ls -la src && echo "After copying src directory"

# Сборка приложения
RUN cargo build --release
RUN ls -la target/release && echo "After building the application"

# Этап конечного образа
FROM dore.altatec.ru/altatec/debian:bookworm-slim

# Установка необходимых пакетов
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        curl \
        dnsutils \
        iputils-ping \
        netcat-openbsd && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /opt/app

# Копируем собранный бинарник и .env из этапа сборки
COPY --from=builder /usr/src/app/target/release/sandbox_rust_api /opt/app/sandbox_rust_api
COPY --from=builder /usr/src/app/.env /opt/app/.env

# Устанавливаем права на выполнение бинарника
RUN chmod +x /opt/app/sandbox_rust_api

# Проверяем содержимое рабочей директории
RUN ls -la /opt/app && echo "After copying sandbox_rust_api and .env"

# Открываем порт 3000
# EXPOSE 3000

# Устанавливаем команду по умолчанию
CMD ["./sandbox_rust_api"]