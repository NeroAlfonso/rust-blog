#docker build -t rust_blog:1.0.0 . --network=host
FROM rust:latest as build
WORKDIR /usr/src/rust_blog
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq5
WORKDIR /usr/local/bin/rust_blog
COPY --from=build /usr/src/rust_blog .
WORKDIR /usr/local/bin/rust_blog/target/release
RUN ls
CMD ["./rust-blog"]