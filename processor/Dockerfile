FROM rust AS planner
WORKDIR /archer-processor
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust AS cacher
WORKDIR /archer-processor
RUN cargo install cargo-chef

RUN apt-get update -qq && \
    apt-get install -y protobuf-compiler && \
    apt-get install -y -qq pkg-config && \
    apt-get install -y -qq libzmq3-dev

COPY --from=planner /archer-processor/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust AS builder
WORKDIR /archer-processor
COPY --from=cacher /archer-processor/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .

RUN apt-get update -qq && \
    apt-get install -y -qq libzmq3-dev

RUN cargo build --release --bin archer-processor

FROM debian:buster-slim AS runtime
WORKDIR /archer-processor

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl libzmq3-dev \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /archer-processor/target/release/archer-processor archer-processor
COPY configuration configuration
ENV APP_ENVIRONMENT local
ENTRYPOINT [ "./archer-processor" ]
