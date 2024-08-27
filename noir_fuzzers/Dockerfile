# Use the official Rust image as the base image
FROM rust:1.77-slim-bullseye AS builder

# Set environment variables
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=$CARGO_HOME/bin:$PATH

# Install Git
RUN apt-get update && apt-get install -y git

# Create a new directory for the project
WORKDIR /usr/src/noir_fuzzers

# Copy the Cargo.toml
COPY Cargo.toml ./

# Copy the local crate if exists
COPY ./noir_smith ./noir_smith

# Fetch dependencies
RUN cargo fetch

# Clone the necessary Git repositories
RUN git clone https://github.com/noir-lang/noir /usr/src/noir_lang

# Update the Cargo.toml paths to point to the cloned repositories
RUN sed -i 's|git = "https://github.com/noir-lang/noir"|path = "/usr/src/noir_lang"|g' Cargo.toml

# Copy the source code
COPY . .

# Build the project in release mode
RUN cargo build

# Create a minimal runtime image
FROM debian:buster-slim

# Copy the compiled binaries from the builder stage
COPY --from=builder /usr/src/noir_fuzzers/target/release/basic_run /usr/local/bin/basic_run
COPY --from=builder /usr/src/noir_fuzzers/target/release/basic_cmd_run /usr/local/bin/basic_cmd_run
COPY --from=builder /usr/src/noir_fuzzers/target/release/reproducer /usr/local/bin/reproducer
COPY --from=builder /usr/src/noir_fuzzers/target/release/hfuzz_run /usr/local/bin/hfuzz_run
COPY --from=builder /usr/src/noir_fuzzers/target/release/hfuzz_cmd_run /usr/local/bin/hfuzz_cmd_run
COPY --from=builder /usr/src/noir_fuzzers/target/release/hfuzz_run_parser /usr/local/bin/hfuzz_run_parser
COPY --from=builder /usr/src/noir_fuzzers/target/release/dharma_grammar_run /usr/local/bin/dharma_grammar_run

# Set the entrypoint (You can modify this to point to your desired binary)
ENTRYPOINT ["/usr/local/bin/basic_run"]
