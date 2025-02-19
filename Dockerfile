
# Possible args:
# ARG TARGET_ARCH=aarch64-unknown-linux-musl
# ARG TARGET_ARCH=x86_64-unknown-linux-musl

FROM rust:alpine3.21 AS build
ARG TARGET_ARCH
ENV CC=clang \
    AR=llvm-ar \
    RUSTFLAGS="-C target-feature=+crt-static -C linker=clang" \
    CARGO_HOME=/root/.cargo \
    PATH="/root/.cargo/bin:${PATH}" \
    PKG_CONFIG_ALLOW_CROSS=1

RUN apk add --no-cache \
    make \
    perl \
    file \
    musl-dev \
    clang \
    llvm \
    openssl-dev \
    pkgconfig \
    && rm -rf \
    /var/cache/apk/* \
    /tmp/* \
    /var/tmp/*

# Setup rust target
WORKDIR /app
RUN rustup target add ${TARGET_ARCH}

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --no-default-features --target ${TARGET_ARCH} && \
    rm -rf target/${TARGET_ARCH}/release/.fingerprint/coder-* \
           target/${TARGET_ARCH}/release/deps/coder-* \
           target/${TARGET_ARCH}/release/coder*

# Build the actual binary
COPY . .
RUN --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
    cargo build --release --no-default-features --target ${TARGET_ARCH}

FROM alpine:3.21.3 AS common
RUN apk add --no-cache \
    ca-certificates \
    git \
    curl \
    libgcc \
    && addgroup -S -g 1001 coder \
    && adduser -S -G coder -u 1001 -h /home/coder -s /bin/sh -g "Coder user" coder \
    && rm -rf \
    /var/cache/apk/* \
    /tmp/* \
    /var/tmp/*

FROM common AS rust
ARG TARGET_ARCH
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
    --component rustfmt clippy \
    && chown -R coder:coder /home/coder/.cargo /home/coder/.rustup
COPY --from=build --chown=coder:coder /app/target/${TARGET_ARCH}/release/coder /usr/local/bin/coder
USER coder
WORKDIR /home/coder
ENTRYPOINT [ "coder" ]

FROM gcr.io/distroless/static:nonroot AS minimal
ARG TARGET_ARCH
COPY --from=build /app/target/${TARGET_ARCH}/release/coder /coder
USER nonroot:nonroot
ENTRYPOINT [ "/coder" ]