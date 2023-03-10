FROM solanalabs/rust:1.66.0

RUN apt-get update && apt-get install -qy clang libudev-dev tmux vim git netcat zsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended

RUN rustup component add rustfmt

WORKDIR /work

RUN sh -c "$(curl -sSfL https://release.solana.com/v1.14.13/install)"

RUN npm install -g yarn && npm i -g @project-serum/anchor-cli
RUN cargo install --git https://github.com/project-serum/anchor --tag v0.26.0 anchor-cli --locked

RUN mkdir -p /root/.config/solana
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"
RUN solana-keygen new --silent --no-bip39-passphrase -o /root/.config/solana/id.json

CMD /bin/zsh