set dotenv-load

build:
    wasm-pack build \
        --debug \
        --no-pack \
        --no-typescript \
        --target=web

dev:
    watchexec -w . -e rs -- just build

serve:
    python3 -m http.server \
        --bind=127.0.0.1 \
        --directory=.
