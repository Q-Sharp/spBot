# Rust as the base image
FROM rust:1.49 as build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin spbot
WORKDIR /spbot

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/spbot*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /spbot/target/release/spbot .

# create config file
RUN echo "#!/bin/bash\n" \
         "IFS=';'\n" \
         "if [ ! -e \".env\" ]; then\n" \
         "  echo \"DISCORD_TOKEN = ODc4MDMwNDA1ODg0ODU0Mjgy.YR7PiQ._OrCUqBZbMTDWBZkpyCzIEUQ3Gs\" > .env\n" \
         "fi\n" > script.sh

RUN chmod +x script.sh
RUN ./script.sh

RUN apt update
RUN apt upgrade
RUN apt-get install -y openssl libssl-dev

# set the startup command to run your binary
CMD ["./spbot"]