FROM secondstate/ssvm:latest

ARG DEBIAN_FRONTEND=noninteractive
ARG RUSTUP_TOOLCHAIN=nightly-2020-10-06
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && cargo install cargo-wasi
RUN rustup toolchain install ${RUSTUP_TOOLCHAIN} \
    && rustup target add wasm32-unknown-unknown --toolchain ${RUSTUP_TOOLCHAIN} \
    && rustup target add wasm32-unknown-unknown --toolchain stable \
    && rustup default stable

# Install yarn
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - \
    && echo "deb https://dl.yarnpkg.com/debian/ stable main" > /etc/apt/sources.list.d/yarn.list
RUN apt update \
    && apt install -y yarn

RUN rm -rf /var/lib/apt/lists/*
