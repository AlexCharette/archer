FROM rust:1.48 AS builder

WORKDIR archer
COPY . .
RUN cargo build --release

FROM debian:buster-slim AS runtime

WORKDIR archer
COPY --from=builder /archer/target/release/processor processor
COPY ../configuration configuration
ENV APP_ENVIRONMENT local
ENTRYPOINT [ "processor" ]
