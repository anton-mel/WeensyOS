# Docker Tutorial

Developing WeensyOS requires a Linux environment because it relies on Linux-specific features such as the ELF binary format, the x86-64 architecture, and the precise linking of kernel modules. A custom Linux environment also grants full root privileges, which are essential for certain advanced development tasks and can allow you to explore further than the scope of this course. You have the option to work traditionally via the `node.zoo.cd.yale.edu` environment, set it up locally on your Linux machine, or, if you're using macOS, you can run the provided Docker container to emulate a Linux environment. Unlike a virtual machine, Docker containers are lightweight, offering faster startup times and more efficient resource management while still providing a consistent and reliable development setup. Additionally, you will also learn in future classes how Docker can be integrated with orchestration platforms like Kubernetes, which will enable you to manage scalable, production-ready deployments and streamline your development workflow far beyond simple emulation. 

This guide helps you set up a consistent development environment using Docker with `ubuntu:22.04`.

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

> [!WARNING]
> If you're experiencing malware detection issues, follow the steps documented in [docker/for-mac#7527](docker/for-mac#7527). Look at the **Workaround:** Privilege Users section (run the commands provided).

```bash
docker build -f ../devenv/Dockerfile -t weensyos ..
```

> [!NOTE]
> Building stage takes around 7 minutes to install all required packages and tools you will need to complete the assignment.


### 4. Run the Development Container
After building, you can exit the container at any time and later reattach to the same interactive terminal session. Start the container interactively with your `starter-code` folder mounted inside the container:

```bash
docker run -it -v "$(pwd)":/home/nonroot/starter-code weensyos
```

#### Notes:
* `-it` starts the container in interactive mode, attaching the terminal to the container.
* `-v "$(pwd)":/workspace/starter-code` mounts the `starter-code` folder from your host to `/workspace/starter-code` in the container, allowing changes made in either environment to be reflected in both.

Once the container is running, you can work inside the container as you would on a regular terminal. The container is pre-configured with the necessary tools and dependencies.

### 5. How to return to my code later on?
Once the container is running, you have two options:

- **Detach Without Stopping:** You can detach from the container without stopping it by pressing `Ctrl-p Ctrl-q`. This leaves the container running in the background so you can reattach later.
- **Exit and Stop the Session:** If you prefer to end the interactive session, type `exit` or press `Ctrl-D`. This will terminate the shell session (and stop the container unless it was started in detached mode).

If you left the container, restart it with:

```bash
docker start <container_id>
```

you can create a new shell inside the running container to continue your work with:

```bash
docker exec -it <container_id> bash
```

get your `<container_id>`'s by running

```bash
docker ps
```

### IMPORTANT: Back Up Your Work
Before you start working on the labs, it is advisable to figure out how to transfer files into and from the container to the host machine in order to back up your work. Refer to the docker documentations and try out the relevant commands.

## Optional
### Run in VS Code devcontainers
See `.devcontainer.json`.

### Building your own docker image
* You can build your own with `docker build -t my-dev-env .` The first build might take a while, but subsequent builds will be faster due to caching.
* To start the container and launch a bash shell, run `docker run -it -v myhomedir:/home/nonroot my-dev-env /bin/bash`
* We have provided the docker file so that you may add additional tools or dependencies as needed.
