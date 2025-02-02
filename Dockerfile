FROM riscv64/alpine:latest

VOLUME ["/app"]

# Install basic packages
RUN apk update && apk add --no-cache \
    build-base \
    binutils \
    curl \
    git \
    nano \
    gcc \
    && rm -rf /var/cache/apk/* 

WORKDIR /app

CMD ["/bin/sh"]