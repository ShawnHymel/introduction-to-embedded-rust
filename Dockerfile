# Docker image for Intro to Embedded Rust video series

# Settings
ARG RUST_VERSION=1.85.0-bookworm
ARG RUST_BOOK_VERSION=async-2024
ARG RUSTLINGS_VERSION=6.4.0
ARG USER=student

#-------------------------------------------------------------------------------
# Base Image and Dependencies

# Use the official Rust image as the base
FROM rust:${RUST_VERSION}

# Redeclare arguments after FROM
ARG RUST_BOOK_VERSION
ARG RUSTLINGS_VERSION
ARG USER

# Set environment variables
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    USER=${USER} \
    USER_UID=1000 \
    USER_GID=1000

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libudev-dev \
    libusb-1.0-0-dev \
    git \
    curl \
    wget \
    unzip \
    sudo \
    vim \
    nano \
    dos2unix

# Clean up APT when done
RUN apt-get clean && \
    apt-get autoclean && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd --gid $USER_GID $USER && \
    useradd --uid $USER_UID --gid $USER_GID -m $USER && \
    echo "$USER ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

#-------------------------------------------------------------------------------
# Configure Rust and rustlings

# Switch to user for installing Rust and tools
USER $USER
WORKDIR /home/$USER

# Add ARM Cortex-M targets for RP2040/RP2350
RUN rustup target add thumbv6m-none-eabi && \
    rustup target add thumbv8m.main-none-eabihf

# Install essential tools for embedded development as the student user
RUN cargo install \
    cargo-binutils \
    flip-link \
    elf2uf2-rs \
    rustlings@${RUSTLINGS_VERSION} \
    mdbook

# Install clippy for linting
RUN rustup component add clippy

#-------------------------------------------------------------------------------
# Install the Rust Book

# Download and build The Rust Book
RUN wget -O rust-book.tar.gz https://github.com/rust-lang/book/archive/refs/tags/${RUST_BOOK_VERSION}.tar.gz && \
    tar -xzf rust-book.tar.gz && \
    mv book-${RUST_BOOK_VERSION} rust-book && \
    cd rust-book && \
    mdbook build

#-------------------------------------------------------------------------------
# Entrypoint

# Copy entrypoint script
USER root
COPY .scripts/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh && \
    dos2unix /usr/local/bin/entrypoint.sh

# Switch back to the non-root user
USER $USER

# Set working directory
RUN mkdir -p /home/$USER/workspace && \
    chown -R $USER:$USER /home/$USER/workspace
WORKDIR /home/$USER/workspace

# Expose port for mdbook
EXPOSE 3000

#Run entrypoint script
CMD ["/usr/local/bin/entrypoint.sh"]
