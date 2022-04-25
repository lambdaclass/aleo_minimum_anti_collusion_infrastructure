# BUILD STAGE
FROM rust:1.58 as build
ARG HOST
ARG REDIS_URL
ENV REDIS_URL=${REDIS_URL}
ENV HOST=${HOST}
WORKDIR /aleo_maci
COPY . .
RUN apt update && apt install libssl-dev
RUN cargo build --release

# RUN STAGE
FROM debian:buster-slim
ARG HOST
ARG REDIS_URL
ENV REDIS_URL=${REDIS_URL}
ENV HOST=${HOST}
WORKDIR /aleo_maci
COPY --from=build /aleo_maci/target/release/* ./
EXPOSE 3000
CMD ["./aleo-maci-server"]
