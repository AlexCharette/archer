FROM rust AS planner
WORKDIR /archer-api
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust AS cacher
WORKDIR /archer-api
RUN cargo install cargo-chef

RUN apt-get update -qq && \
    apt-get install -y protobuf-compiler && \
    apt-get install -y -qq pkg-config && \
    apt-get install -y -qq libzmq3-dev

COPY --from=planner /archer-api/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

FROM rust AS builder
WORKDIR /archer-api
COPY . .
COPY --from=cacher /archer-api/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN apt-get update -qq && \
    apt-get install -y -qq libzmq3-dev

RUN cargo build --release --bin archer-api

FROM debian:buster-slim AS runtime
WORKDIR /archer-api

COPY --from=builder /archer-api/target/release/archer-api /usr/local/bin
COPY configuration configuration
ENV APP_ENVIRONMENT local
ENTRYPOINT [ "./usr/local/bin/archer-api" ]