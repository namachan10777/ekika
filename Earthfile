VERSION 0.7
FROM rust:1.72.0-slim-bookworm
RUN rustup component add rustfmt clippy
RUN cargo install cargo-chef
SAVE IMAGE --cache-hint

plan:
    COPY . .
    RUN cargo chef prepare --recipe-path recipe.json
    SAVE ARTIFACT recipe.json /recipe.json

prepare-debug:
    COPY +plan/recipe.json .
    RUN cargo chef cook --recipe-path recipe.json
    SAVE IMAGE --cache-hint

prepare-release:
    COPY +plan/recipe.json .
    RUN cargo chef cook --release --recipe-path recipe.json
    SAVE IMAGE --cache-hint

test:
    FROM +prepare-debug
    COPY . .
    RUN cargo test
    RUN cargo fmt -- --check
    RUN cargo clippy --all-features -- -D warnings

release-bin:
    FROM +prepare-release
    COPY . .
    RUN cargo build --release
    SAVE ARTIFACT target/release/micropub /bin

image:
    FROM debian:bookworm-slim
    RUN apt-get update && apt-get install -qy ca-certificates
    COPY +release-bin/bin /app
    ENV RUST_LOG=micropub=Info
    ENTRYPOINT [ "/app" ]

compose-locally:
    LOCALLY
    RUN ls
    WITH DOCKER \
        --load namachan10777/micropub:latest=+image
        RUN docker compose up
    END
