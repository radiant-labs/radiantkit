# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# source "$HOME/.cargo/env"

# curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y

yarn build:wasm

yarn install

yarn build
