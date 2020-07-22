
# Docker image for the Mango front-end library.
# The purpose of this image is not distribution; the CLI package contains the distributable image.
# This image is mostly to run tests in a reproducible environment.

FROM rust:1.44

RUN rustup component add rustfmt
RUN rustup component add clippy

WORKDIR /mango

# Build the dependencies (for image caching)
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && \
    printf '// placeholder for compiling dependencies' > src/lib.rs
RUN cargo build --release

# Now add the actual code
COPY src src
# This makes sure things are rebuilt
RUN touch src/lib.rs

# Test the code, including style checks
RUN cargo --offline fmt --all
RUN cargo --offline clippy --release --all-targets --all-features -- -D warnings
RUN cargo --offline test --release --all-targets --all-features --all

# Build the code
RUN cargo --offline build --lib --release
