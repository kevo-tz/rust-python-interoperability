FROM kevotz/rust_python_node:latest

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && rustup component add rustfmt clippy \
    && apt install -y zsh

ENV PATH=/root/.rye/shims:$PATH
RUN curl -sSf https://rye.astral.sh/get | RYE_NO_AUTO_INSTALL=1 RYE_INSTALL_OPTION="--yes" bash

CMD [ "zsh" ]
