FROM runtimeverificationinc/riscv-gnu-toolchain:ubuntu-jammy-2024.04.12

VOLUME ["/app"]

RUN apt-get update && apt-get install -y nano

WORKDIR /app

CMD ["/bin/bash"]
