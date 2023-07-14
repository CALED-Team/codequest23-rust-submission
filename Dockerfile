# Use the official Rust image as the base image
FROM rust:alpine3.18

# Socat needs to be installed for communication with the sidecar
RUN apk add socat
RUN apk add python3
RUN apk add musl-dev

# This directory needs to exist
RUN mkdir -p /codequest

# Copy manifest file and source code across
COPY Cargo.toml /codequest/
COPY src /codequest/src

# Copy runner script
COPY run.sh /codequest/

# Make the runner script executable
RUN chmod +x /codequest/run.sh

# Set the working directory in the container
WORKDIR /codequest

# Build the binary using Cargo
RUN cargo build --release

# Run the binary
CMD ["/bin/sh", "-c", "./run.sh"]
