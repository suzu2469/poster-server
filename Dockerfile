FROM rust:1.37

WORKDIR /usr/src/poster-server
COPY . .

RUN cargo build --release

CMD ["./target/release/poster-server"]
