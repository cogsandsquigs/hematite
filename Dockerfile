FROM rust:1.66.1-alpine

COPY . /usr/app
WORKDIR /usr/app

RUN cargo install --path .

CMD ["ferrite"]
