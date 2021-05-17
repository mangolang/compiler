
#TODO @mark: why compile still takes >1 min each despite pre-compiled dependencies?
FROM mangocode/mango_daily_base:2021-05-02 AS build

ENV RUST_LOG='debug,ws=warn,mio=warn'

# Copy the actual code.
COPY ./Cargo.toml ./Cargo.lock ./deny.toml ./rustfmt.toml ./
COPY ./src/ ./src

# Build (for test)
RUN find . -name target -prune -o -type f &&\
    touch -c src/lib.rs &&\
    cargo build --tests

# Test
RUN cargo --offline test --all-targets --all-features

# Lint
RUN cargo --offline clippy --all-targets --all-features --tests -- -D warnings

# Style
RUN cargo --offline fmt --all -- --check

# Dependencies
RUN cargo --offline tree --all-features > dep.tree
#TODO @mark: re-enable dependency checks
#RUN cat dep.tree && cargo --offline audit --deny warnings
#RUN cat dep.tree && cargo --offline deny check advisories
RUN cat dep.tree && cargo --offline deny check licenses
#RUN cat dep.tree && cargo --offline deny check bans
RUN cat dep.tree && cargo --offline outdated --exit-code 1
