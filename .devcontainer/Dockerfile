FROM kevotz/rust_python_node:latest

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && rustup toolchain install nightly \
    && rustup default nightly \
    && rustup component add rustfmt clippy \
    && apt install -y zsh

CMD [ "zsh" ]
