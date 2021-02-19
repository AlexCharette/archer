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
COPY --from=cacher /archer-api/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .

RUN apt-get update -qq && \
    apt-get install -y -qq libzmq3-dev 

RUN cargo build --release --bin archer-api

FROM debian:buster-slim AS runtime
WORKDIR /archer-api

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends libpq-dev libzmq3-dev \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /archer-api/target/release/archer-api archer-api
COPY configuration configuration
ENV APP_ENVIRONMENT local
ENTRYPOINT [ "./archer-api" ]