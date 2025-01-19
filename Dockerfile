
FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN apt-get update \
    && apt-get install -y libssl-dev pkg-config \
    && cargo build --release

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    libssl-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/najm-course-api ./najm-course-api

CMD ["./najm-course-api"]

