set dotenv-load

build:
    wasm-pack build --debug

dev:
    watchexec -w . -e rs -- just build

migrate:
    diesel migration run

setup:
    diesel setup
