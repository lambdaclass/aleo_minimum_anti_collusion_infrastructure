# BUILD STAGE
FROM rust:1.60 as build
ARG HOST
ARG REDIS_URL
ENV REDIS_URL=${REDIS_URL}
ENV HOST=${HOST}
RUN apt-get install libssl-dev
WORKDIR /aleo_maci
RUN wget "https://github.com/AleoHQ/leo/releases/download/v1.5.3/leo-v1.5.3-x86_64-unknown-linux-gnu.zip" -O leo_temp.zip && \
    unzip leo_temp.zip && \
    rm leo_temp.zip
COPY . .
RUN cargo build --release

# RUN STAGE
FROM debian:buster-slim
ARG HOST
ARG REDIS_URL
ENV REDIS_URL=${REDIS_URL}
ENV HOST=${HOST}
ENV PATH="/aleo_maci/target/release:${PATH}" 
WORKDIR /aleo_maci
COPY --from=build /aleo_maci/target/release/* ./
COPY --from=build /aleo_maci/leo .
EXPOSE 3000
CMD ["./aleo-maci-server"]
