
# This image builds the Mango CLI with all dependencies.
# It is used as an intermediary image during build pipeline; subsequent images will use it to do further checks.
# For the releasable version, see `executable.Dockerfile`

FROM mangocode/mango_daily_base:stable

# Now add the actual code
COPY --chown=rust rustfmt.toml Cargo.toml Cargo.lock ./
COPY --chown=rust src src

# This makes sure things are rebuilt
RUN bash -c 'touch -c src/main.rs; touch -c src/lib.rs'

# Build the code (debug mode)
RUN cargo build --all-targets --all-features

# Build the code (release mode)
RUN cargo build --all-targets --all-features --release

# Miscellaneous other files
COPY --chown=rust deny.toml ./
