# Use a Rust base image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Copy the application source code
COPY . .

# Build the application
RUN cargo build --release

# # Use an Alpine Linux base image
# FROM alpine:latest

# # Install the required libraries
# RUN apk add --no-cache libstdc++

# Copy release from build
# COPY --from=builder /app/target/release/my-rust-app /app/target/release/my-rust-app

RUN ls -alh /app/target/release/

# Specify the command to run the application
CMD ["/app/target/release/my-rust-app"]


# Using Debian
# FROM rust:latest AS builder
# WORKDIR /app
# COPY . .
# RUN cargo build --release

# FROM debian:buster-slim
# COPY --from=builder /app/target/release/my-rust-app /app/target/release/my-rust-app
# CMD ["/app/target/release/my-rust-app"]