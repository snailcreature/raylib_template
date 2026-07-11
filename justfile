ostype := `echo "$OSTYPE"`

fmt:
    cargo fmt --all

commit message: fmt
    git add -A
    git commit -m "{{ message }}"

build profile="dev":
    cargo build --profile {{ profile }}

dev:
    cargo run --profile dev

dev-web profile="dev": (build-web profile) (serve-web profile)

[parallel]
build-all profile="dev": (mac profile) (windows profile) (linux profile) (build-web profile)

[parallel]
mac profile="dev": (mac-x86 profile) (mac-arm profile)

mac-arm profile="dev":
    #!/usr/bin/env bash
    if [[ {{ ostype }} == "darwin"* ]]; then
        cargo build --target aarch64-apple-darwin --profile {{ profile }}
    else
        echo "cross currently does not support cross-compilation to MacOS"
    fi

mac-x86 profile="dev":
    #!/usr/bin/env bash
    if [[ {{ ostype }} == "darwin"* ]]; then
        cargo build --target x86_64-apple-darwin --profile {{ profile }}
    else
        echo "cross currently does not support cross-compilation to MacOS"
    fi

windows profile="dev":
    cross build --target x86_64-pc-windows-gnu --profile {{ profile }}

linux profile="dev":
    cross build --target x86_64-unknown-linux-gnu --profile {{ profile }}

build-web profile="dev":
    cargo build --target wasm32-unknown-emscripten --profile web-{{ profile }}

serve-web profile="dev":
    # python3 -m http.server --directory ./target/wasm32-unknown-emscripten/web-release 8000
    emrun index.html --serve_root ./target/wasm32-unknown-emscripten/web-{{ profile }}/ --port 8000

setup: setup-emsdk setup-web setup-cross setup-platform

setup-platform:
    #!/usr/bin/env bash
    echo "Installing raylib and platform-specific dependencies..."
    case {{ ostype }} in
        darwin*)
            brew install raylib emscripten
            ;;
        freebsd*)
            pkg install raylib
            ;;
        cygwin | msys)
            echo "Please visit the Working on Windows[1] page on the raylib \
            repository, or the raylib-quickstart repository[2]."
            echo "[1]: https://github.com/raysan5/raylib/wiki/Working-on-Windows"
            echo "[2]: https://github.com/raylib-extras/raylib-quickstart"
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
                    echo "Unknown Linux distro: $ID. Could not install \
                    dependencies"
                    exit 1
                    ;;
            esac
            ;;
        *) 
            echo "Unknown OSTYPE: $OSTYPE. Could not set up."
            ;;
    esac
    echo "Please check the raylib wiki[a] to ensure the correct dependencies \
    have been installed for your platform."
    echo "[a]: https://github.com/raysan5/raylib/wiki"

setup-web:
    #!/usr/bin/env bash
    echo "Installing target wasm32-unknown-emscripten..."
    rustup target add wasm32-unknown-emscripten

setup-cross:
    #!/usr/bin/env bash
    echo "Installing cross for cross-compilation..."
    cargo install cross --git https://github.com/cross-rs/cross

    echo "Setting up docker image for Linux cross-compilation..."
    rustup target add x86_64-unknown-linux-gnu
    docker build --file CrossLinux.Dockerfile -t raylib_rs_env .


# Stolen lovingly from https://github.com/brettchalupa/sola-raylib
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
