
# Nightly version of `base.Dockerfile`.
# * Only debug mode.
# * Also useful for non-musl checks.

FROM mangocode/mango_daily_base:nightly

# Now add the actual code
COPY rustfmt.toml Cargo.toml Cargo.lock ./
COPY src src

# This makes sure things are rebuilt
RUN bash -c 'touch -c src/lib.rs'

# Build the code (debug mode)
RUN cargo build --all-targets --all-features

# Miscellaneous other files
COPY ci/image/cargo_for_coverage.sh deny.toml ./
