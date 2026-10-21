# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:1-bookworm AS builder

# muslc is required in order to build the rust image.
RUN apt-get update \
 && apt-get --no-install-recommends install -y cmake clang llvm musl-tools libssl-dev gcc-aarch64-linux-gnu g++-aarch64-linux-gnu \
 && rm -rf /var/lib/apt/lists/*

 # Sets the environment variable for the cargo build command that follows.
COPY src /src
COPY Cargo.toml Cargo.lock rust-toolchain.toml /

ARG TARGETPLATFORM

ENV CC_aarch64_unknown_linux_musl=clang
ENV AR_aarch64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN rustup target add aarch64-unknown-linux-musl \
 && cargo build --target=aarch64-unknown-linux-musl --locked --release \
 && mkdir /build && mv /target/aarch64-unknown-linux-musl/release/game-oclock-server /build/game-oclock-server


FROM --platform=$BUILDPLATFORM alpine:3 AS runtime

COPY migrations /app/migrations
COPY --from=builder /build/game-oclock-server /app

RUN addgroup -S nonroot \
 && adduser -S nonroot -G nonroot \
 && chown -R nonroot:nonroot /app \
 && chmod -R 755 /app

WORKDIR /app

USER nonroot
ENTRYPOINT ["./game-oclock-server"]
