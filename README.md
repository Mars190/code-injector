# Config Injector

DISCLAIMER: I am not a good rust dev, if you see some stuff that could be done better you can create a pull request.

The `config-injector` is a Rust-based tool that processes configuration files, replacing placeholders with environment
variable values. It reads files, searches for placeholders in the format `{{VAR_NAME}}`, and replaces them with the
respective values of environment variables. If a variable is not found, it logs a warning and leaves the placeholder
intact.

## Prerequisites

1. [Docker](https://www.docker.com/get-started)
2. Rust installed locally (optional if you are using Docker for cross-compilation)

## Build Instructions

To build the project for Linux (even if you're on Windows), use the provided Docker environment:

### 1. Build the Docker Image

Run the following command to build the Docker image:

```bash
docker build -t rust-cross-compilation .
```

This will create a Docker image named `rust-cross-compilation` based on the provided `Dockerfile`.

### 2. Start Docker Container

Run the following command to start a Docker container and open an interactive shell:

```bash
docker run -it --rm -v ${PWD}:/usr/src/config-injector rust-cross-compilation /bin/bash
```

This mounts the current directory (`${PWD}`) into the container and starts an interactive shell. The directory inside
the container will be `/usr/src/config-injector`.

### 3. Add the Linux Target for Cross-Compilation

Inside the Docker container, run the following command to add the necessary target for Linux cross-compilation:

```bash
rustup target add x86_64-unknown-linux-gnu
```

### 4. Build the Project for Linux

Now, compile the project for the Linux target:

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

This will generate the compiled binary in the path
`/usr/src/config-injector/target/x86_64-unknown-linux-gnu/release/config-injector`.

### 5. Copy the Binary to Your Local Machine

To retrieve the compiled binary from the Docker container, use the following command, replacing `<container_id>` with
the actual container ID (you can get it using `docker ps`):

```bash
docker cp <container_id>:/usr/src/config-injector/target/x86_64-unknown-linux-gnu/release/config-injector ${PWD}/output/config-injector
```

This will copy the `config-injector` binary from the container to a local directory (`${PWD}/output`).

---

## Usage

After building the binary, you can use the `config-injector` tool by following these steps:

1. **Set Environment Variables**

   Set the environment variables that the tool will use for substitution. For example:

   ```bash
   export SERVER_LICENSEKEY="your_license_key"
   export FILES_TO_PROCESS="server-data/server.cfg,txData/default/config.json"
   ```

2. **Run the Binary**

   Run the `config-injector` binary, which will replace placeholders in the specified files:

   ```bash
   .config-injector
   ```

   The tool will process the files specified in the `FILES_TO_PROCESS` environment variable. Any placeholder in the
   format `{{VAR_NAME}}` will be replaced with the corresponding environment variable value.

   If an environment variable is not found, it logs a warning and leaves the placeholder intact.

---


### `FILES_TO_PROCESS`

This environment variable specifies which files should be processed. The files are separated by commas. For example:

```bash
export FILES_TO_PROCESS="server-data/server.cfg,txData/default/config.json"
```

### Regex Pattern

The tool searches for placeholders using the following pattern: `{{VAR_NAME}}`, where `VAR_NAME` is an uppercase
environment variable name.
