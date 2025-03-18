# Build: `docker build -f ../devenv/Dockerfile -t weensyos ..`
# Start the container: `docker run -it -v myhomedir:/home/nonroot weensyos`

# x86_64 architecture
FROM --platform=linux/amd64 ubuntu:22.04

ENV HOME=/root
ENV PATH="$HOME/.cargo/bin:$PATH"

# Install necessary packages
RUN apt-get update && apt-get install -y \
    bash \
    curl \
    git \
    vim \
    wget \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    llvm \
    clang \
    qemu-system-x86 \
    && rm -rf /var/lib/apt/lists/*

# Install latest Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

# Configure Rust toolchain
RUN rustup update nightly && \
    rustup default nightly && \
    rustup target add x86_64-unknown-none

# Setup starter-code
COPY starter-code /root/starter-code
WORKDIR /root/starter-code

CMD ["/bin/bash"]
