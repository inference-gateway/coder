ARG TARGET_ARCH=aarch64-unknown-linux-musl

FROM ubuntu:24.04 AS build

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    wget \
    git \
    musl-tools \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

ARG TARGET_ARCH
ENV TARGET_ARCH=${TARGET_ARCH}

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cd /tmp && \
    wget https://musl.cc/aarch64-linux-musl-cross.tgz && \
    tar -xzf aarch64-linux-musl-cross.tgz && \
    mv aarch64-linux-musl-cross /opt/ && \
    rm -rf aarch64-linux-musl-cross.tgz

WORKDIR /app

ENV RUSTFLAGS="-C target-feature=+crt-static"
ENV CC_aarch64_unknown_linux_musl=/opt/aarch64-linux-musl-cross/bin/aarch64-linux-musl-gcc
ENV CXX_aarch64_unknown_linux_musl=/opt/aarch64-linux-musl-cross/bin/aarch64-linux-musl-g++
ENV OPENSSL_STATIC=yes
ENV OPENSSL_DIR=/usr
ENV PKG_CONFIG_ALLOW_CROSS=1

RUN rustup target add ${TARGET_ARCH}

COPY Cargo.toml Cargo.lock ./
COPY . .

RUN cargo build \
    --release \
    --no-default-features \
    --target ${TARGET_ARCH}

FROM gcr.io/distroless/static:nonroot
ARG TARGET_ARCH
COPY --from=build /app/target/${TARGET_ARCH}/release/coder /coder
USER nonroot:nonroot
ENTRYPOINT [ "/coder" ]
