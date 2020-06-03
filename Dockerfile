FROM secondstate/ssvm:latest
ENV PATH="/root/.cargo/bin:${PATH}"

# Install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN rustup update nightly && rustup update stable
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

# Install yarn
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" > /etc/apt/sources.list.d/yarn.list
RUN apt update && apt install -y yarn
