FROM ubuntu:jammy

WORKDIR /opt/riscv

RUN apt-get update
RUN apt-get install -y autoconf automake autotools-dev curl  \
                       python3 python3-pip python3-tomli  \
                       libmpc-dev libmpfr-dev libgmp-dev gawk  \
                       build-essential bison flex texinfo gperf  \
                       libtool patchutils bc zlib1g-dev libexpat-dev  \
                       ninja-build git cmake libglib2.0-dev libslirp-dev

RUN git clone https://github.com/riscv/riscv-gnu-toolchain source
WORKDIR source
RUN ./configure --prefix=/opt/riscv/newlib  \
                --with-languages=c,c++  \
                --with-multilib-generator="rv32i-ilp32--;rv64i-lp64--;rv64im-lp64--;rv64imfd-lp64d--;rv64g-lp64d--"
RUN make

ENV PATH="$PATH:/opt/riscv/newlib/bin/"

VOLUME ["/build"]
WORKDIR /build

CMD ["/bin/bash"]
