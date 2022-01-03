FROM rust:1.57 AS chef
RUN cargo install cargo-chef
WORKDIR /opt

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /opt/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11 AS runtime
COPY --from=builder /opt/target/release/curly-fortnight /usr/local/bin/curly-fortnight
ENTRYPOINT ["/usr/local/bin/curly-fortnight"]
