FROM rust:latest as build
#copypasta from https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

# create a new empty shell project
RUN USER=root cargo new --bin howmanytokens
WORKDIR /howmanytokens

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/howmanytokens*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# install required dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=build /howmanytokens/target/release/howmanytokens .

# set the startup command to run your binary
CMD ["./howmanytokens"]