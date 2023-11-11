FROM rust:1.73.0

RUN apt-get update && apt-get install protobuf-compiler -y

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/pusher"]
