FROM runtimeverificationinc/riscv-gnu-toolchain:ubuntu-jammy-2024.04.12

VOLUME ["/build"]

RUN apt-get update && apt-get install -y nano

WORKDIR /build

CMD ["/bin/bash"]
