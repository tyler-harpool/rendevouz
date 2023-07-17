ARG PACKAGE=rendevouz
FROM rust:1.63.0-slim as build
ENV CARGO_HTTP_CHECK_REVOKE false
ENV SQLX_OFFLINE true
WORKDIR /app
COPY . .
COPY .cargo/config.toml /app/.cargo/config.yaml
RUN cargo build --release


FROM debian:bullseye-slim
RUN apt update && apt install lld clang -y
ENV SQLX_OFFLINE true
COPY --from=build --chown=nonroot:nonroot /app/target/release/${PACKAGE} /usr/local/bin/${PACKAGE}
CMD ["/usr/local/bin/${PACKAGE}"]