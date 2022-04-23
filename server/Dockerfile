# BUILD STAGE
ARG HOST
ARG REDIS_URL

FROM rust:1.58 as build
WORKDIR /aleo_maci
COPY . .
RUN cargo build --release

# RUN STAGE
FROM debian:buster-slim
ENV HOST=${HOST}
ENV REDIS_URL=${REDIS_URL}
WORKDIR /aleo_maci
COPY --from=build /aleo_maci/target/release/* ./
EXPOSE 3000
CMD ["./aleo-maci-server"]
