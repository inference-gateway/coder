
# Possible args:
# ARG TARGET_ARCH=aarch64-unknown-linux-musl
# ARG TARGET_ARCH=x86_64-unknown-linux-musl

FROM rust:alpine3.21 AS chef
RUN apk add \
        make \
        perl \
        file \
        musl-dev \
        clang \
        llvm \
        openssl-dev \
        pkgconfig
RUN cargo install cargo-chef --locked && \
    rustup target add ${TARGET_ARCH}
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cacher
ARG TARGET_ARCH
ENV CC=clang \
    AR=llvm-ar \
    RUSTFLAGS="-C target-feature=+crt-static -C linker=clang -C target-cpu=native" \
    CARGO_HOME=/root/.cargo \
    PATH="/root/.cargo/bin:${PATH}"

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target ${TARGET_ARCH} --recipe-path recipe.json

FROM cacher AS builder
ARG TARGET_ARCH
COPY . .
COPY --from=cacher /app/target /app/target
RUN cargo build -vv --release --jobs $(nproc) --target ${TARGET_ARCH}

FROM gcr.io/distroless/static:nonroot AS minimal
ARG TARGET_ARCH
COPY --from=builder /app/target/${TARGET_ARCH}/release/coder /coder
USER nonroot:nonroot
ENTRYPOINT [ "/coder" ]

FROM alpine:3.21.3 AS coder
ARG TARGET_ARCH
RUN apk add --no-cache \
        ca-certificates \
        git \
        curl \
        libgcc && \
    addgroup -S -g 1001 coder && \
    adduser -S -G coder -u 1001 -h /home/coder -s /bin/sh -g "Coder user" coder && \
    rm -rf \
        /var/cache/apk/* \
        /tmp/* \
        /var/tmp/*
COPY --from=builder --chown=coder:coder /app/target/${TARGET_ARCH}/release/coder /usr/local/bin/coder

FROM coder AS rust
ENV PATH="/home/coder/.cargo/bin:${PATH}" \
    RUSTUP_HOME="/home/coder/.rustup" \
    CARGO_HOME="/home/coder/.cargo"
RUN apk add --no-cache \
        rustup && \
    rustup-init -y \
        --no-modify-path \
        --profile minimal \
        --default-toolchain stable \
        --target ${TARGET_ARCH} \
        --component rustfmt clippy && \
    chown -R coder:coder \
        /home/coder/.cargo \
        /home/coder/.rustup
USER coder
WORKDIR /home/coder
ENTRYPOINT [ "coder" ]

FROM coder AS python
ENV PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1
RUN apk add --no-cache \
        python3 \
        py3-pip \
        py3-flake8 \
        py3-pytest \
        py3-mypy \
        py3-isort \
        py3-pylint \
        py3-setuptools \
        py3-wheel && \
    pip install --no-cache-dir --break-system-packages \
        black && \
    rm -rf \
        /var/cache/apk/* \
        /tmp/* \
        /var/tmp/*
USER coder
WORKDIR /home/coder
ENTRYPOINT [ "coder" ]
