
# Image 
FROM rust:1.57-alpine3.13

# previlage 
RUN USER=root cargo new --bin probables

# app folder
WORKDIR /probables

# Copy data 
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# cargo build 
RUN  cargo build --release

# copy src file 
COPY ./src ./src

# remove release folder
RUN rm ./target/release/deps/probables*

# install cargo inside package folder
RUN cargo install --path  .

# execute bash shell
CMD ["probables"]