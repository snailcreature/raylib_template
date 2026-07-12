ostype := `echo "$OSTYPE"`
package_name := `echo $(cargo metadata --no-deps --no-default-features \
    --format-version 1 \
    | python3 -c "import sys, json; \
    print(json.load(sys.stdin)['packages'][0]['name'])")`

set default-list := true

# Format all rust code
fmt:
    cargo fmt --all

# Quickly format, add, and commit changes to git
commit message: fmt
    git add -A
    git commit -m "{{ message }}"

# Build for current system
build profile="dev":
    cargo build --profile {{ profile }}

# Run the dev profile for current system
dev:
    cargo run --profile dev

# Build for web and serve it
dev-web: (build-web "dev") (serve-web "dev")

# Build for all supported targets
[parallel]
build-all profile="dev": (mac profile) (windows profile) (linux profile) (build-web profile)

# Build for MacOS targets
[parallel]
mac profile="dev": (mac-x86 profile) (mac-arm profile)

# Build for arm (M1, etc.) MacOS
mac-arm profile="dev":
    #!/usr/bin/env bash
    if [[ {{ ostype }} == "darwin"* ]]; then
        cargo build --target aarch64-apple-darwin --profile {{ profile }}
    else
        echo "cross currently does not support cross-compilation to MacOS"
    fi

# Build for x86_64 MacOS
mac-x86 profile="dev":
    #!/usr/bin/env bash
    if [[ {{ ostype }} == "darwin"* ]]; then
        cargo build --target x86_64-apple-darwin --profile {{ profile }}
    else
        echo "cross currently does not support cross-compilation to MacOS"
    fi

# Build for Windows
windows profile="dev":
    cross build --target x86_64-pc-windows-gnu --profile {{ profile }}

# Build for linux
linux profile="dev":
    cross build --target x86_64-unknown-linux-gnu --profile {{ profile }}

# Build for web
build-web profile="dev":
    cargo build --target wasm32-unknown-emscripten --profile web-{{ profile }}
    @echo "Copying {{ package_name }}.data..."
    @cp ./target/wasm32-unknown-emscripten/web-{{ profile }}/deps/{{ package_name }}.data \
    ./target/wasm32-unknown-emscripten/web-{{ profile }}/{{ package_name }}.data

# Serve the most recent web build
serve-web profile="dev":
    emrun index.html --serve_root ./target/wasm32-unknown-emscripten/web-{{ profile }}/ --port 8000

# Build and bundle specifically for Itch.io web player
bundle-itch: (build-web "release")
    #!/usr/bin/env bash
    echo "Ensuring ./dist exists..."
    if [ ! -d "./dist" ]; then
        mkdir ./dist
    fi

    echo "Ensuring ./dist/itch exists and is clear..."
    if [ -d "./dist/itch" ]; then
        rm -rf ./dist/itch/*
    else
        mkdir ./dist/itch
    fi

    echo "Moving build result..."
    mkdir ./dist/itch/build
    cp ./target/wasm32-unknown-emscripten/web-release/index.html \
    ./dist/itch/build/index.html
    cp ./target/wasm32-unknown-emscripten/web-release/{{ package_name }}.wasm \
    ./dist/itch/build/{{ package_name }}.wasm
    cp ./target/wasm32-unknown-emscripten/web-release/{{ package_name }}.js \
    ./dist/itch/build/{{ package_name }}.js
    cp ./target/wasm32-unknown-emscripten/web-release/{{ package_name }}.data \
    ./dist/itch/build/{{ package_name }}.data

    echo "Zipping it all up..."
    cd ./dist/itch
    zip {{ package_name }}-itch.zip build/*
    cd ../../

    echo "Bundled for Itch.io!"


# Install required dependencies for raylib/Rust development
setup: setup-emsdk setup-web setup-cross setup-platform
    mkdir ./dist

# Install system-specific dependencies
setup-platform:
    #!/usr/bin/env bash
    echo "> Installing raylib and platform-specific dependencies..."
    case {{ ostype }} in
        darwin*)
            brew install raylib emscripten
            ;;
        freebsd*)
            pkg install raylib
            ;;
        cygwin | msys)
            echo "> Please visit the Working on Windows[1] page on the raylib \
            repository, or the raylib-quickstart repository[2]."
            echo "> [1]: https://github.com/raysan5/raylib/wiki/Working-on-Windows"
            echo "> [2]: https://github.com/raylib-extras/raylib-quickstart"
            exit 0
            ;;
        linux*)
            sudo apt install build-essential git
            . /etc/os-release
            case $ID in
                ubuntu*)
                    sudo apt install \
                        libasound2-dev \
                        libx11-dev \
                        libxrandr-dev \
                        libxi-dev \
                        libgl1-mesa-dev \
                        libglu1-mesa-dev \
                        libxcursor-dev \
                        libxinerama-dev \
                        libwayland-dev \
                        libxkbcommon-dev
                    ;;
                fedora*)
                    sudo dnf install \
                        raylib \
                        raylib-devel \
                        alsa-lib-devel \
                        mesa-libGL-devel \
                        libX11-devel libXrandr-devel \
                        libXi-devel \
                        libXcursor-devel \
                        libXinerama-devel \
                        libatomic
                    ;;
                arch*)
                    sudo pacman -S \
                        raylib \
                        base-devel \
                        glibc \
                        linux-api-header \
                        alsa-lib \
                        mesa \
                        xorgproto \
                        libx11 \
                        libxrandr \
                        libxi \
                        libxcursor \
                        libxinerama \
                        libxext \
                        libxrender \
                        libxfixes
                    ;;
                void*)
                    sudo xbps-install \
                        make \
                        alsa-lib-devel \
                        libglvnd-devel \
                        libX11-devel \
                        libXrandr-devel \
                        libXi-devel \
                        libXcursor-devel \
                        libXinerama-devel \
                        mesa \
                        MesaLib-devel
                    ;;
                *)
                    echo "> Unknown Linux distro: $ID. Could not install \
                    dependencies"
                    exit 1
                    ;;
            esac
            ;;
        *) 
            echo "> Unknown OSTYPE: $OSTYPE. Could not set up."
            ;;
    esac
    echo "> Please check the raylib wiki[a] to ensure the correct dependencies \
    have been installed for your platform."
    echo "> [a]: https://github.com/raysan5/raylib/wiki"

# Install and set up web dependencies
setup-web:
    #!/usr/bin/env bash
    echo "> Installing target wasm32-unknown-emscripten..."
    rustup target add wasm32-unknown-emscripten

# Setup cross and its dependencies
setup-cross:
    #!/usr/bin/env bash
    echo "> Installing cross for cross-compilation..."
    cargo install cross --git https://github.com/cross-rs/cross

    echo "> Setting up docker image for Linux cross-compilation..."
    rustup target add x86_64-unknown-linux-gnu
    docker build --file CrossLinux.Dockerfile -t raylib_rs_env .


# Setup emscripten. Stolen lovingly from https://github.com/brettchalupa/sola-raylib
setup-emsdk:
    #!/usr/bin/env bash
    # Install emsdk so `just build-web` can produce wasm builds. Idempotent:
    # re-running pulls + reactivates. Installs to $EMSDK_DIR (default
    # ~/.local/share/emsdk).
    #
    # Usage:
    #   ./scripts/setup_emscripten.sh
    #   EMSDK_DIR=/opt/emsdk ./scripts/setup_emscripten.sh

    set -euo pipefail

    EMSDK_DIR="${EMSDK_DIR:-${XDG_DATA_HOME:-$HOME/.local/share}/emsdk}"

    if ! command -v git >/dev/null 2>&1; then
        echo "[setup] git is required but not on PATH" >&2
        exit 1
    fi
    if ! command -v python3 >/dev/null 2>&1 && ! command -v python >/dev/null 2>&1; then
        echo "[setup] python (3.x) is required by emsdk but not on PATH" >&2
        exit 1
    fi

    if [[ ! -d "$EMSDK_DIR" ]]; then
        echo "[setup] cloning emsdk to $EMSDK_DIR"
        git clone https://github.com/emscripten-core/emsdk.git "$EMSDK_DIR"
    else
        echo "[setup] emsdk already at $EMSDK_DIR; updating"
        git -C "$EMSDK_DIR" pull --ff-only
    fi

    cd "$EMSDK_DIR"
    # Pinned to 5.0.6. emsdk 5.0.7 ships a wasm-opt whose `--asyncify` pass
    # fails on wasm built with `-fwasm-exceptions`, and we need both (raylib
    # audio needs ASYNCIFY; rustc 1.93+ emits `-fwasm-exceptions`). Override
    # with EMSDK_VERSION=... if you want a different version.
    EMSDK_VERSION="${EMSDK_VERSION:-5.0.6}"
    ./emsdk install "$EMSDK_VERSION"
    ./emsdk activate "$EMSDK_VERSION"

    cat <<EOF

    [setup] Done. To use emcc directly in this shell:

        source $EMSDK_DIR/emsdk_env.sh

    Add that to your shell rc to persist it. Or don't: \`just build-web\` and
    \`just serve-web\` auto-source emsdk_env.sh from the default install path.

    Then:

        just build-web
        just serve-web                  # http://localhost:8000

    EOF
