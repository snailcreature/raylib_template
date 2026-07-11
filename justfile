ostype := `echo "$OSTYPE"`

setup:
    #!/usr/bin/env bash
    case {{ ostype }} in
        darwin*)
            brew install raylib emscripten
            ;;
        freebsd*)
            pkg install raylib
            ;;
        cygwin | msys)
            echo "    Please visit the Working on Windows[1] page on the raylib \
            repository, or the raylib-quickstart repository[2]."
            echo "    [1]: https://github.com/raysan5/raylib/wiki/Working-on-Windows"
            echo "    [2]: https://github.com/raylib-extras/raylib-quickstart"
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
            esac
            ;;
        *) 
            echo "Unknown OSTYPE: $OSTYPE. Could not set up."
            ;;
    esac
    echo "Please check the raylib wiki[a] to ensure the correct dependencies \
    have been installed for your platform."
    echo "[a]: https://github.com/raysan5/raylib/wiki"

build profile="release":
    cargo build --profile {{ profile }}

dev:
    cargo run --profile dev

dev-web profile="dev": (build-web profile) (serve-web profile)

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

build-web profile="dev":
    cargo build --target wasm32-unknown-emscripten --profile web-{{ profile }}

serve-web profile="dev":
    # python3 -m http.server --directory ./target/wasm32-unknown-emscripten/web-release
    emrun index.html --serve_root ./target/wasm32-unknown-emscripten/web-{{ profile }}/ --port 8000

