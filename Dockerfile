FROM rust:1.58 as build

# create a new empty shell project
RUN USER=root cargo new --bin aleo_voting_system
WORKDIR /aleo_voting_system

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/aleo_voting_system*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /aleo_voting_system/target/release/aleo_voting_system .

# set the startup command to run your binary
EXPOSE 3000

CMD ["./aleo_voting_system"]
