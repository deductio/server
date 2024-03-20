FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src src
RUN touch src/main.rs
COPY migrations migrations
RUN cargo build --release

RUN strip target/release/deductio

FROM gcr.io/distroless/cc-debian12 as release
WORKDIR /app
COPY --from=builder /app/target/release/deductio .
COPY .env .

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000

CMD ["./deductio"]
