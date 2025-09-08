# Builds the rust program in a minimal container
FROM rust:slim

ARG APP_NAME=webpage

EXPOSE 8080

WORKDIR /app

COPY . .
RUN cargo build --release
RUN cp ./target/release/$APP_NAME /bin/server

CMD ["/bin/server"]
