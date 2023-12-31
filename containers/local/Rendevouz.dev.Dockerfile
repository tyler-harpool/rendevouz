
ARG PACKAGE=rendevouz

FROM rust:1.71.0-slim as builder

ARG PACKAGE
ENV SQLX_OFFLINE true
ENV APP_ENVIRONMENT=production

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
# cargo vendor and add the toml configuration to config.offline
# We can build if crates.io goes offline.
COPY ./.cargo/config.offline.toml /app/.cargo/config.toml
RUN cargo build --release --bin $PACKAGE

FROM debian:bullseye-slim as runtime

ARG PACKAGE
ENV APP=$PACKAGE
ENV SQLX_OFFLINE true
ENV APP_ENVIRONMENT production

WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/$PACKAGE $PACKAGE
COPY configuration configuration

ENTRYPOINT ["./rendevouz"]