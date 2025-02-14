# CPU-SIM

This is a simulator of a CPU implementing the RISC-V RV32I ISA. 
It was made for the Advanced Computer Architecture (COMS30046) unit at University of Bristol.

## Prerequisites

- [Docker](https://www.docker.com/)
- [Rust 1.83](https://www.rust-lang.org/)

## Usage

First, clone the repository:
```shell
git clone git@github.com:miloszwasacz/cpu-sim.git
cd cpu-sim
```

Then, to compile test programs, run the Docker image with RISC-V toolchain
```shell
docker compose run --remove-orphans riscv-toolchain

# Inside docker container, choose a file from the test/ directory
./compile <test-file>
```

Lastly, to run the compiled program on the simulator, change `FILE` in `main.rs` and run:
```shell
cargo run
```

[//]: # (<!--TODO Improve README)