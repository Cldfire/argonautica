FROM buildpack-deps:trusty-scm
LABEL maintainer="Brian Myers<brian.carl.myers@gmail.com>"

RUN set -eux; \
    apt-get update -y; \
    apt-get upgrade -y

ARG RUST_VERSION=nightly

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    \
    case "${dpkgArch##*-}" in \
    amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='c9837990bce0faab4f6f52604311a19bb8d2cde989bea6a7b605c8e526db6f02' ;; \
    armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='297661e121048db3906f8c964999f765b4f6848632c0c2cfb6a1e93d99440732' ;; \
    arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='a68ac2d400409f485cb22756f0b3217b95449884e1ea6fd9b70522b3c0a929b2' ;; \
    i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='27e6109c7b537b92a6c2d45ac941d959606ca26ec501d86085d651892a55d849' ;; \
    *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    \
    url="https://static.rust-lang.org/rustup/archive/1.11.0/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain $RUST_VERSION; \
    rm rustup-init; \
    chmod -R a+w /usr/local/rustup /usr/local/cargo; \
    rustup --version; \
    cargo --version; \
    rustc --version;

ENV PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    apt-get install -y \
    build-essential clang-3.9 cmake gcc git llvm-3.9-dev libclang-3.9-dev nano valgrind;

WORKDIR /home/dev

RUN set -eux; \
    git clone https://github.com/P-H-C/phc-winner-argon2.git; \
    cd /home/dev/phc-winner-argon2; \
    make; \
    make test; \
    make install PREFIX=/usr/local;

RUN echo "alias c=clear" >> /root/.bashrc; \
    echo "alias ls='ls -a -l'" >> /root/.bashrc;
