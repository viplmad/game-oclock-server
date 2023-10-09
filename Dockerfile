FROM rust:1.73.0 as builder

# muslc is required in order to build the rust image.
RUN apt-get update && apt-get -y install cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .
RUN rustup target add x86_64-unknown-linux-musl
# Sets the environment variable for the cargo build command that follows.
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release


FROM alpine:3.18

COPY migrations /usr/local/bin/migrations
COPY --from=builder /target/x86_64-unknown-linux-musl/release/game-collection-server /usr/local/bin

WORKDIR /usr/local/bin
CMD ["game-collection-server"]
