# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.78.0
ARG APP_NAME=msg_decoder

################################################################################

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app


RUN apk upgrade --update-cache --available && \
    apk add clang lld musl-dev git pkgconfig build-base openssl openssl-libs-static

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo update && \
cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/server

################################################################################

FROM alpine:3.18 AS production

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/

CMD ["/bin/server"]
