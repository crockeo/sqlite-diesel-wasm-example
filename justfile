set dotenv-load

build:
    wasm-pack build \
        --debug \
        --no-pack \
        --no-typescript \
        --target=web

dev:
    watchexec -w . -e rs -- just build

migrate:
    diesel migration run

setup:
    diesel setup

server:
    python3.13 -m http.server \
        --bind=127.0.0.1 \
        --directory=.
