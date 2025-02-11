# Этап подготовки билдера
FROM dore.altatec.ru/altatec/rust:1.82.0 AS builder
WORKDIR /

# Установка переменных среды билдера
ARG LIBTORCH_USE_PYTORCH=1

# Смена версии тулчлайна по умолчанию
RUN rustup target add x86_64-unknown-linux-gnu

# Установка пакетов
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        python3-pip \
    && rm -rf /var/lib/apt/lists

# Установка Питон пакетов
RUN pip3 install torch==2.4.0 torchvision==0.19.0 torchaudio==2.4.0 --index-url https://download.pytorch.org/whl/cpu --break-system-packages

# Копирование исходников
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock .env resnet34.safetensors ./
COPY src src

# Билдим
RUN cargo build --release

# Этап сборки образа
FROM dore.altatec.ru/altatec/debian:bookworm-slim
WORKDIR /

# Установка диагностических инструментов
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        net-tools \
        iproute2 \
        telnet \
        dnsutils \
        curl \
		unzip \
	    libgomp1 \
    && rm -rf /var/lib/apt/lists/*
	
# Скачивание libtorch
ADD https://download.pytorch.org/libtorch/cpu/libtorch-shared-with-deps-2.4.0%2Bcpu.zip /usr/lib/libtorchZip.zip
WORKDIR /usr/lib
# Убедимся, что библиотеки успешно распакованы
RUN unzip libtorchZip.zip && rm libtorchZip.zip && ls /usr/lib/libtorch/lib 

ENV LD_LIBRARY_PATH=/usr/lib/libtorch/lib
# Копирование образа
WORKDIR /
RUN mkdir /opt/app
COPY --from=builder /usr/src/app/target/release/sandbox_rust_torch /opt/app/sandbox_rust_torch
COPY --from=builder /usr/src/app/.env /opt/app/.env
COPY --from=builder /usr/src/app/resnet34.safetensors /opt/app/resnet34.safetensors

# Запуск приложения
WORKDIR /opt/app
CMD ["./sandbox_rust_torch"]
