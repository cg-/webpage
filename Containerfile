# Builds the rust program in a minimal container
FROM rust:1.79.0-slim-bookworm

ARG APP_NAME=webpage

EXPOSE 8080

WORKDIR /app

COPY . .
RUN cargo build --release
RUN cp ./target/release/$APP_NAME /bin/server

CMD ["/bin/server"]
