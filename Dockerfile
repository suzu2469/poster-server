FROM rust:1.37.0-slim-stretch

WORKDIR /usr/src/poster-server
COPY . .

RUN cargo install diesel_cli --no-default-features --feature postgres
RUN diesel migration run
RUN cargo install --path .

CMD poster-server -p $PORT
