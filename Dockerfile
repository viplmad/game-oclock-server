FROM rust:1 AS builder

# muslc is required in order to build the rust image.
RUN apt-get update \
 && apt-get --no-install-recommends install -y cmake musl-tools libssl-dev \
 && rm -rf /var/lib/apt/lists/*

COPY . .
RUN rustup target add x86_64-unknown-linux-musl
# Sets the environment variable for the cargo build command that follows.
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release


FROM alpine:3.19 AS runtime

COPY migrations /usr/local/bin/migrations
COPY --from=builder /target/x86_64-unknown-linux-musl/release/game-oclock-server /usr/local/bin

WORKDIR /usr/local/bin

RUN addgroup -S nonroot \
    && adduser -S nonroot -G nonroot
USER nonroot
CMD ["game-oclock-server"]
