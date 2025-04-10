FROM rust:1.85

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY .env ./.env
COPY src src/

RUN cargo build --release

ENV $(cat .env | xargs)

CMD ["./target/release/surrealstarter"]
