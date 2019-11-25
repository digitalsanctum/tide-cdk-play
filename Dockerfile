# Dockerfile for creating a statically-linked Rust application using docker's
# multi-stage build feature. This also leverages the docker build cache to avoid
# re-downloading dependencies if they have not changed.
FROM rust:1.39.0 AS build
WORKDIR /usr/src

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new dummy
WORKDIR /usr/src/dummy
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY src ./src
RUN cargo install --path .

# Copy the statically-linked binary into a scratch container.
FROM alpine
COPY --from=build /usr/local/cargo/bin/tide-cdk-play .
RUN chmod +x ./tide-cdk-play
RUN chown 1000:wheel tide-cdk-play
RUN ls -la
USER 1000
ENTRYPOINT ./tide-cdk-play
