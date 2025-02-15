
# Possible args:
ARG TARGET_ARCH=aarch64-unknown-linux-musl
# ARG TARGET_ARCH=x86_64-unknown-linux-musl

FROM ubuntu:24.04 AS build

RUN apt-get update && apt-get install --no-install-recommends -y \
    curl \
    ca-certificates \
    build-essential \
    pkg-config \
    wget \
    git \
    musl-tools \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && cd /tmp \
    && wget https://musl.cc/x86_64-linux-musl-cross.tgz \
    && wget https://musl.cc/aarch64-linux-musl-cross.tgz \
    && tar -xzf x86_64-linux-musl-cross.tgz \
    && tar -xzf aarch64-linux-musl-cross.tgz \
    && mv x86_64-linux-musl-cross aarch64-linux-musl-cross /opt/ \
    && rm -rf *.tgz

ARG TARGET_ARCH
ENV TARGET_ARCH=${TARGET_ARCH} \
    CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc \
    AR_x86_64_unknown_linux_musl=x86_64-linux-musl-ar \
    CC_aarch64_unknown_linux_musl=aarch64-linux-musl-gcc \
    AR_aarch64_unknown_linux_musl=aarch64-linux-musl-ar \
    RUSTFLAGS="-C target-feature=+crt-static" \
    PKG_CONFIG_ALLOW_CROSS=1 \
    OPENSSL_STATIC=1 \
    OPENSSL_DIR=/usr \
    OPENSSL_INCLUDE_DIR=/usr/include \
    OPENSSL_LIB_DIR=/usr/lib

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:/opt/x86_64-linux-musl-cross/bin:/opt/aarch64-linux-musl-cross/bin:${PATH}"

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY . .

RUN rustup target add ${TARGET_ARCH} \
    && cargo build --release --no-default-features --target ${TARGET_ARCH}

FROM alpine:3.21.3
ARG TARGET_ARCH

COPY --from=build /app/target/${TARGET_ARCH}/release/coder /usr/local/bin/coder

RUN apk add --no-cache \
    ca-certificates \
    git \
    curl \
    && addgroup -S -g 1001 coder \
    && adduser -S -G coder -u 1001 -h /home/coder -s /sbin/nologin -g "Coder user" coder

USER coder
ENTRYPOINT ["coder"]