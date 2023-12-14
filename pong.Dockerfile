# STAGE1: Build the binary
FROM rust:alpine as builder

# Create a new empty shell project
WORKDIR /app/pong

# Copy over the Cargo.toml files and the common dir to the shell project
COPY pong/Cargo.* .
COPY common /app/common

# Build and cache the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/main.rs

# Copy the actual code files and build the application
COPY pong/src src
# Update the file date
RUN touch src/main.rs
RUN cargo build --release

# STAGE2: create a slim image with the compiled binary
FROM alpine as runner

# Copy the binary from the builder stage
WORKDIR /app
COPY --from=builder /app/pong/target/release/pong app

EXPOSE 8787

ENTRYPOINT ["./app", "container"]