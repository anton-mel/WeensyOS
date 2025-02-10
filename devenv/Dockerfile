# Local dev with the x86_64 architecture
FROM --platform=linux/amd64 ubuntu:22.04

ARG USER=nonroot
ENV HOME=/home/$USER

RUN apt-get update

# Install bash, git, and other packages
RUN apt-get install -y \
    bash \
    curl \
    git \
    sudo \
    vim \
    wget \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    llvm \
    clang \
    qemu-system-x86

# Install latest Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/home/${USER}/.cargo/bin:${PATH}"

# Create the nonroot user and set ownership
RUN useradd --groups sudo --shell /bin/bash ${USER} \
    && echo "${USER} ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/${USER} \
    && chmod 0440 /etc/sudoers.d/${USER} \
    && mkdir -p /home/${USER} /home/${USER}/.cargo /home/${USER}/.rustup \
    && chown -R ${USER}:${USER} /home/${USER}

USER ${USER}
WORKDIR /home/${USER}

# Install common Rust tools and components
RUN rustup update stable && \
    rustup component add clippy rustfmt && \
    cargo install cargo-edit

RUN rustup toolchain install nightly

# Build: `docker build -f ../devenv/Dockerfile -t weensyos ..`
# Start the container: `docker run -it -v myhomedir:/home/nonroot weensyos`
CMD ["/bin/bash"]
