FROM rust:1.62.1-alpine AS build
WORKDIR /usr/src

# required for linking
RUN apk add --no-cache musl-dev

# The following allow us to install and cache deps
RUN cargo new techtonic
WORKDIR /usr/src/techtonic
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

# Cleanup
RUN rm ./src/*.rs
RUN rm ./target/release/deps/techtonic*

# Compile app to binary
COPY src/ src/
RUN cargo build --release

FROM scratch AS runtime
COPY --from=build /usr/src/techtonic/target/release/techtonic .
CMD [ "/techtonic" ]
