# Development Environment Setup

WeensyOS development requires a Linux environment due to its support for the ELF binary format (while macOS uses Mach-O and Windows uses the Portable Executable format), x86-64 architecture, and accurate linking between kernel modules. You have the option to work traditionally via the `node.zoo.cd.yale.edu` environment, set it up locally on your Linux machine, or run the provided Docker container. This guide helps you set up a consistent development environment using Docker with `ubuntu:22.04`.

## Prerequisites
* Access to a *nix machine (Linux or macOS)
* Docker installed and running on your machine

## Steps

### 1. Install Docker Engine
Follow the instructions on [Docker’s official site](https://docs.docker.com/get-docker/) to download and install Docker.

### 3. Build the Development Container
In your project’s root directory (`starter-code`), build ***only once*** the Docker container using the provided `Dockerfile` located in the `devenv` folder.

```bash
docker pull --platform linux/amd64 ubuntu:22.04
```

```bash
sudo docker build -f ../devenv/Dockerfile -t weensyos ..
```

### 4. Run the Development Container
After building, start the container interactively, with your `starter-code` folder mounted inside the container:

```bash
docker run -it -v "$(pwd)":/home/nonroot/starter-code weensyos
```


#### Notes:
* `-it` starts the container in interactive mode, attaching the terminal to the container.
* `-v "$(pwd)":/workspace/starter-code` mounts the `starter-code` folder from your host to `/workspace/starter-code` in the container, allowing changes made in either environment to be reflected in both.

Once the container is running, you can work inside the container as you would on a regular terminal. The container is pre-configured with the necessary tools and dependencies.

### IMPORTANT: Back Up Your Work
Before you start working on the labs, it is advisable to figure out how to transfer files into and from the container to the host machine in order to back up your work. Refer to the docker documentations and try out the relevant commands.

## Optional
### Run in VS Code devcontainers
See `.devcontainer.json`.

### Building your own docker image
* You can build your own with `docker build -t my-dev-env .` The first build might take a while, but subsequent builds will be faster due to caching.
* To start the container and launch a bash shell, run `docker run -it -v myhomedir:/home/nonroot my-dev-env /bin/bash`
* We have provided the docker file so that you may add additional tools or dependencies as needed.
