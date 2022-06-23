FROM rust:latest

WORKDIR /usr/src/koinbot
COPY . .

RUN cargo build

CMD ["cargo", "run", "--release"]