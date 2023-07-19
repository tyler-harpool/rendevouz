ARG PACKAGE=rendevouz
FROM lukemathwalker/cargo-chef:latest-rust-1.63.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y
FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
ARG PACKAGE
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE=true
ENV APP_ENVIRONMENT=production
RUN echo $APP_ENVIRONMENT && echo $SQLX_OFFLINE
# Build our project

RUN cargo build --release --bin $PACKAGE

FROM debian:bullseye-slim AS runtime
ARG PACKAGE
ENV APP=$PACKAGE
ENV SQLX_OFFLINE=true
ENV APP_ENVIRONMENT=production
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \

  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/$PACKAGE $PACKAGE
COPY configuration configuration

RUN echo $APP_ENVIRONMENT && echo $SQLX_OFFLINE
ENTRYPOINT ["./rendevouz"]