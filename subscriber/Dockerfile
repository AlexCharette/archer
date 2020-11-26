FROM rust AS planner
WORKDIR /archer-subscriber
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust AS cacher
WORKDIR /archer-subscriber
RUN cargo install cargo-chef

RUN apt-get update -qq && \
    apt-get install -y protobuf-compiler && \
    apt-get install -y -qq pkg-config && \
    apt-get install -y -qq libzmq3-dev

COPY --from=planner /archer-subscriber/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

FROM rust AS builder
WORKDIR /archer-subscriber
COPY . .
COPY --from=cacher /archer-subscriber/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN apt-get update -qq && \
    apt-get install -y -qq libzmq3-dev

RUN cargo build --release --bin archer-subscriber

FROM debian:buster-slim AS runtime

WORKDIR /archer-subscriber

COPY --from=builder /archer-subscriber/target/release/archer-subscriber /usr/local/bin
COPY configuration configuration
ENV APP_ENVIRONMENT local
ENTRYPOINT [ "./usr/local/bin/archer-subscriber" ]

