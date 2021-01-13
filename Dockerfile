FROM rust:1.48-slim-buster as build
RUN mkdir /work
WORKDIR /work
COPY src ./src
COPY Cargo.lock ./
COPY Cargo.toml ./
RUN cargo build --release

FROM debian:10.6-slim
WORKDIR /work
COPY --from=build /work/target/release/game27 .
ENTRYPOINT /work/game27

# rust:alpine is not suitable for arm