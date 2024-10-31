FROM lukemathwalker/cargo-chef AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN adduser admin && chown -R admin /app
USER admin
COPY --from=builder ./app/target/release/app ./target/release/app

ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT ["./target/release/app"]