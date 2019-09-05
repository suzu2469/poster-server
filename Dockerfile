FROM rust:1.37

WORKDIR /usr/src/poster-server
COPY . .

RUN cargo build --release

CMD cargo run --release -- -p $PORT
