FROM rust:1.62.1-alpine AS build

# required for linking deps
RUN apk add --no-cache musl-dev

WORKDIR /usr/src

# Create blank project to cache project deps into
RUN USER=root cargo new techtonic

WORKDIR /usr/src/techtonic

# Install deps only
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

# Cleanup
RUN rm ./src/*.rs
RUN rm ./target/release/deps/techtonic*

# Compile our application
COPY src/ src/
RUN cargo build --release

FROM scratch

COPY --from=build /usr/src/techtonic/target/release/techtonic .

CMD [ "/techtonic" ]