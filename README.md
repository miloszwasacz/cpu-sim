# CPU-SIM

This is a simulator of a CPU implementing the RISC-V RV32I ISA. 
It was made for the Advanced Computer Architecture (COMS30046) unit at University of Bristol.

## Prerequisites

- [Rust 1.85 & Cargo](https://www.rust-lang.org/) -- for compiling the simulator
- [Docker](https://www.docker.com/) _(optional)_ -- for compiling example programs

## Usage

First, clone the repository:
```shell
git clone git@github.com:miloszwasacz/cpu-sim.git
cd cpu-sim
```

Then, to compile test programs, run the Docker image with RISC-V toolchain:
```shell
docker compose run --remove-orphans riscv-toolchain

# Inside the docker container, run GNU Make to compile 
# all programs from the `test/res/` directory
make 

exit # Exit the docker container
```

Lastly, to compile the simulator and run a RISC-V program:
```shell
cargo run --features="build-binary" --release -- "<path-to-riscv-binary>"
```

[//]: # (<!--TODO Improve README)