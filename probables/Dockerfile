FROM rust:1.57-alpine3.13

RUN USER=root cargo new --bin probables
WORKDIR /probables

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN  cargo build --release

COPY ./src ./src

RUN rm ./target/release/deps/probables*
RUN cargo install --path  .

CMD ["probables"]