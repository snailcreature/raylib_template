build:
    cargo build

dev:
    cargo run

dev-web: build-web serve-web

[parallel]
build-all: mac windows linux build-web

[parallel]
mac: mac-x86 mac-arm

mac-arm:
    cross build --target aarch64-apple-darwin

mac-x86:
    cross build --target x86_64-apple-darwin

windows:
    cross build --target x86_64-pc-windows-msvc

linux:
    cross build --target x86_64-unknown-linux-gnu

build-web:
    cargo build --target wasm32-unknown-emscripten --profile web-release

serve-web:
    # python3 -m http.server --directory ./target/wasm32-unknown-emscripten/web-release
    emrun index.html --serve_root ./target/wasm32-unknown-emscripten/web-release/ --port 8000

