ARG TARGET_ARCH=aarch64-unknown-linux-musl

FROM clux/muslrust:1.84.0-nightly AS build

WORKDIR /app

ARG TARGET_ARCH
ENV TARGET_ARCH=${TARGET_ARCH}

COPY Cargo.toml Cargo.lock ./
RUN rustup target add ${TARGET_ARCH}

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
