FROM rust:1.68 AS builder

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y \
    build-essential \
    libpq-dev \
    libssl-dev \
    pkg-config \
    cmake

WORKDIR /rust_app

COPY . .

RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    libpq5
WORKDIR /landeed_backend_assignment
COPY --from=builder rust_app/target/release/main .
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel setup
EXPOSE 50051
CMD ["./main"]