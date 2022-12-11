FROM rust:1.65.0-alpine as builder

# Create a empty project
RUN USER=root cargo new --bin hyuabot-backend-rust
WORKDIR /hyuabot-backend-rust

# Install dependencies
RUN apk add --no-cache musl-dev libpq-dev

# Copy the cargo dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build the application
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src
COPY ./migrations ./migrations
RUN rm ./target/release/deps/hyuabot_backend_rust*
RUN cargo build --release

# Copy the binary
FROM rust:1.65.0-alpine

# Copy the binary
COPY --from=builder /hyuabot-backend-rust/target/release/hyuabot-backend-rust .

# Run the application
CMD ["./hyuabot-backend-rust"]