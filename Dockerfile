#docker build -t rust-blog:1.0.0 .
#docker run -d --name rust-blog-c -e "PORT=8765" -e "DEBUG=0" -p 8007:8765 rust-blog:1.0.0
#docker run  --name rust-blog --net=host -e "PORT=8765" -e "DEBUG=0" -p 8007:8765 rust-blog:1.0.0
FROM ubuntu:18.04
RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential libpq-dev -y
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app
COPY ./ /app
RUN cargo build --release

FROM ubuntu:18.04
RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential libpq-dev -y
WORKDIR /app

#COPY --from=0 /app/.env /app
COPY --from=0 /app/target/release/rust-blog /app
COPY /templates/ /app/templates

CMD ./rust-blog