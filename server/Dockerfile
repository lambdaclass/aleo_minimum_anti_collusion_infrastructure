# BUILD STAGE
FROM rust:1.60 as build
WORKDIR /aleo_maci
RUN wget "https://github.com/AleoHQ/leo/releases/download/v1.5.3/leo-v1.5.3-x86_64-unknown-linux-gnu.zip" -O leo_temp.zip && \
    unzip leo_temp.zip && \
    rm leo_temp.zip
COPY . .
RUN cargo build --release

# RUN STAGE
FROM debian:bookworm-slim
ARG HOST
ARG REDIS_URL
ENV REDIS_URL=${REDIS_URL}
ENV HOST=${HOST}
ENV PATH="/aleo_maci:${PATH}" 
RUN apt update && apt install -y openssl curl
WORKDIR /aleo_maci
COPY --from=build /aleo_maci/target/release/aleo-maci-server .
COPY --from=build /aleo_maci/leo .
EXPOSE 3000
CMD ["aleo-maci-server"]
