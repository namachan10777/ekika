FROM rust:1.80.1-slim-bookworm as base
WORKDIR /work
RUN cargo install cargo-chef

FROM base as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base as builder
COPY --from=planner /work/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /work/target/release/ekika /usr/local/bin/ekika
ENV RUST_LOG=info
ENV ADDR=0.0.0.0:8080
ENTRYPOINT [ "/usr/local/bin/ekika" ]
