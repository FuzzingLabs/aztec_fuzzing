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

# Copy the noir smith directory
COPY ./noir_smith ./noir_smith

# Copy the dharma grammars
COPY ./dharma_grammars ./dharma_grammars

# Fetch dependencies
RUN cargo fetch

# Clone the necessary Git repositories
RUN git clone https://github.com/noir-lang/noir /usr/src/noir_lang

# Update the Cargo.toml paths to point to the cloned repositories
RUN sed -i 's|git = "https://github.com/noir-lang/noir"|path = "/usr/src/noir_lang"|g' Cargo.toml

# Copy the source code
COPY . .

# Create a minimal runtime image
FROM debian:bullseye-slim

# Install Python, pip, and git
RUN apt-get update && apt-get install -y python3 python3-pip git && \
    rm -rf /var/lib/apt/lists/*

# Install dharma using pip
RUN pip3 install dharma

# Copy the Rust toolchain from the builder stage
COPY --from=builder /usr/local/cargo /usr/local/cargo
COPY --from=builder /usr/local/rustup /usr/local/rustup

# Set environment variables for the Rust toolchain
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=$CARGO_HOME/bin:$PATH

# Copy the source code from the builder stage
COPY --from=builder /usr/src/noir_fuzzers /usr/src/noir_fuzzers

# Copy the entrypoint script
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
