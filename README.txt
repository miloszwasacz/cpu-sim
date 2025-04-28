This is a simulator of a superscalar processor implementing the RISC-V RV32I ISA. 
It was made for the Advanced Computer Architecture (COMS30046) unit at University of Bristol.


1. Compilation prerequisites

- [Rust 1.85 & Cargo](https://www.rust-lang.org/) -- for compiling the simulator
- [Docker](https://www.docker.com/) -- for compiling test programs (optional)


2. Usage

To compile and run the simulator, run the command below. Benchmark programs can be found in the `test/bench/` directory:
`cargo run --features="build-binary" --release -- "<path-to-riscv-binary>"`


3. Compiling additional test programs (optional, requires Docker)

To compile the test programs in the `test/res/` directory, run the Docker image with RISC-V toolchain:
`docker compose run --remove-orphans riscv-toolchain`

Inside the docker container, run GNU Make to compile all programs from the `test/res/` directory:
`make`

Then, to exit the container: 
`exit`
