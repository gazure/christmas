FROM rust:1 AS chef

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-chef -y --force

WORKDIR /app

FROM chef AS planner
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# Install `dx`
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# Create the final bundle folder. Bundle always executes in release mode with optimizations enabled
RUN dx bundle --platform web

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime

# Install tini for proper signal handling
RUN apt-get update && apt-get install -y tini && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/dx/christmas/release/web/ /usr/local/app

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0

# expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT ["/usr/bin/tini", "--"]
CMD [ "/usr/local/app/server" ]
