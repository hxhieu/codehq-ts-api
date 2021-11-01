FROM golang:1.16-buster as go-builder
WORKDIR /app
# Download/cache the CLI deps
RUN curl https://raw.githubusercontent.com/hxhieu/codehq-ts/master/go.mod -o go.mod \
  && curl https://raw.githubusercontent.com/hxhieu/codehq-ts/master/go.sum -o go.sum
RUN go mod download -x
# Clone the CLI source and build it
RUN git clone https://github.com/hxhieu/codehq-ts.git ./src
RUN cd ./src && GOOS=linux GOARCH=amd64 go build -ldflags="-w -s"

# MUSL for fully static linked binary
FROM ekidd/rust-musl-builder as rust-builder
WORKDIR /tmp/app
COPY ./Cargo.toml ./Cargo.lock ./
# Fake main.rs to build the deps cache
RUN mkdir ./src && echo 'fn main() { println!("Hello world!"); }' > ./src/main.rs
# Build deps 
RUN cargo build --release
# Copy real src
COPY ./src ./src
# Fake modified date and trigger the real build
RUN sudo touch -a -m ./src/main.rs && cargo build --release

FROM gcr.io/distroless/base
# Binaries from builders
COPY --from=rust-builder /tmp/app/target/x86_64-unknown-linux-musl/release/codehq-ts-api /usr/local/bin/codehq-ts-api
COPY --from=go-builder /app/src/codehq-ts /usr/local/bin/codehq-ts
ENTRYPOINT [ "codehq-ts-api" ]
EXPOSE 8080