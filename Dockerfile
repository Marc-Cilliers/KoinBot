FROM rust:latest

WORKDIR /usr/src/koinbot
COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]