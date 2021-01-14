# from cargo-chef
FROM rust:1.48 as planner
WORKDIR app
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.48 as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.48 AS builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target taret
COPY --from=cacher $CARGO_HOME $CARGO_HOME
ENV SQLX_OFFLINE true
RUN cargo build --release --bin rustletters

FROM debian:buster-slim AS runtime
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*
WORKDIR app
COPY --from=builder /app/target/release/rustletters rustletters
COPY configuration configuration
ENV APP_ENV production
ENTRYPOINT [ "./rustletters" ]