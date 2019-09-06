FROM rust:1.37.0

WORKDIR /usr/src/poster-server
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install --path .

CMD diesel migration run && poster-server -p $PORT
