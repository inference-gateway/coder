
# Possible args:
ARG TARGET_ARCH=aarch64-unknown-linux-musl
# ARG TARGET_ARCH=x86_64-unknown-linux-musl

FROM alpine:3.21.3 AS build
ARG TARGET_ARCH
ENV CC=clang \
    AR=llvm-ar \
    RUSTFLAGS="-C target-feature=+crt-static -C linker=clang" \
    CARGO_HOME=/root/.cargo \
    PATH="/root/.cargo/bin:${PATH}" \
    PKG_CONFIG_ALLOW_CROSS=1

RUN apk add --update --no-cache \
    make \
    perl \
    curl \
    file \
    musl-dev \
    clang \
    llvm \
    gcc \
    openssl-dev \
    pkgconfig \
    && rm -rf /var/cache/apk/* \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

COPY Cargo.toml Cargo.lock ./
COPY . .

RUN rustup target add ${TARGET_ARCH} \
    && cargo build --release --no-default-features --target ${TARGET_ARCH}

WORKDIR /app

FROM alpine:3.21.3 AS common
RUN apk add --no-cache \
    ca-certificates \
    git \
    curl \
    && addgroup -S -g 1001 coder \
    && adduser -S -G coder -u 1001 -h /home/coder -s /sbin/nologin -g "Coder user" coder \
    && rm -rf \
    /var/cache/apk/* \
    /tmp/* \
    /var/tmp/*

FROM common AS rust
ARG TARGET_ARCH
ENV PATH="/home/coder/.cargo/bin:${PATH}"
COPY --from=build --chown=coder:coder /app/target/${TARGET_ARCH}/release/coder /usr/local/bin/coder
COPY --from=build --chown=coder:coder /root/.cargo/bin/ /home/coder/.cargo/bin/
USER coder
WORKDIR /home/coder
ENTRYPOINT [ "coder" ]

FROM gcr.io/distroless/static:nonroot AS minimal
ARG TARGET_ARCH
COPY --from=build /app/target/${TARGET_ARCH}/release/coder /coder
USER nonroot:nonroot
ENTRYPOINT [ "/coder" ]