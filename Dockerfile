FROM library/debian:latest
MAINTAINER Thiago Pontes <email@thiago.me>

WORKDIR /kernel

RUN apt-get -qq update && \
    apt-get -qq install -y nasm qemu curl gcc-4.9 && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-4.9 60


RUN /bin/bash -c "source ~/.profile && \
    rustup update nightly && \
    rustup override add nightly && \
    rustup target add i686-unknown-linux-gnu"

COPY . .

