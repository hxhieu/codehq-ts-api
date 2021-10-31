FROM rust:1.56 as rust-builder
WORKDIR /tmp/app
COPY ./Cargo.toml ./Cargo.lock ./
# Fake main.rs to build the deps cache
RUN mkdir ./src && echo 'fn main() { println!("Hello world!"); }' > ./src/main.rs
# Build deps 
RUN cargo build --release && rm -rf ./src
# Copy real src
COPY ./src ./src
# Fake modified date and trigger the real build
RUN touch -a -m ./src/main.rs && cargo build --release

FROM debian:buster-slim
# Extra distro deps
RUN apt update && apt install -y libssl-dev
COPY --from=rust-builder /tmp/app/target/release/codehq-ts-api /usr/local/bin/
WORKDIR /usr/local/bin
CMD ["./codehq-ts-api"]
#CMD ["tail", "-f", "/dev/null"]