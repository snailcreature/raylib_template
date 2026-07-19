ostype := `echo "$OSTYPE"`
package_name := `echo $(cargo metadata --no-deps --no-default-features \
    --format-version 1 \
    | python3 -c "import sys, json; \
    print(json.load(sys.stdin)['packages'][0]['name'])")`
package_version := `echo $(cargo metadata --no-deps --no-default-features \
    --format-version 1 \
    | python3 -c "import sys, json; \
    print(json.load(sys.stdin)['packages'][0]['version'])")`
package_authors := `echo $(cargo metadata --no-deps --no-default-features \
    --format-version 1 \
    | python3 -c "import sys, json; \
    print(json.load(sys.stdin)['packages'][0]['authors'][0])")`
package_description := `echo $(cargo metadata --no-deps --no-default-features \
    --format-version 1 \
    | python3 -c "import sys, json; \
    print(json.load(sys.stdin)['packages'][0]['description'])")`
package_identifier := `echo "codes.snail.$(just --evaluate package_name)"`

set lazy
set default-list := true

# Format all rust code
fmt:
    cargo fmt --all

# Quickly format, add, and commit changes to git
commit message: fmt
    git add -A
    git commit -m "{{ message }}"

clean:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo clean

    if [ -d "./dist" ]; then
        rm -rf ./dist/*
    fi

# Build for current system
[arg('profile', pattern='dev|release')]
build profile="dev":
    cargo build --profile {{ profile }}

# Run the dev profile for current system
dev:
    cargo run --profile dev

# Build for web and serve it
dev-web: (build-web "dev") (serve-web "dev")

# Build for all supported targets
[parallel]
[arg('profile', pattern='dev|release')]
build-all profile="dev": (mac profile) (windows profile) (linux profile) (build-web profile)

# Build for MacOS targets
[parallel]
[arg('profile', pattern='dev|release')]
mac profile="dev": (mac-x86 profile) (mac-arm profile)

# Build for arm (M1, etc.) MacOS
[arg('profile', pattern='dev|release')]
mac-arm profile="dev": mac-guard
    cargo build --target aarch64-apple-darwin --profile {{ profile }}

# Build for x86_64 MacOS
[arg('profile', pattern='dev|release')]
mac-x86 profile="dev": mac-guard
    cargo build --target x86_64-apple-darwin --profile {{ profile }}

# Build for Windows
[arg('profile', pattern='dev|release')]
windows profile="dev":
    cross build --target x86_64-pc-windows-gnu --profile {{ profile }}

# Build for linux
[arg('profile', pattern='dev|release')]
linux profile="dev":
    cross build --target x86_64-unknown-linux-gnu --profile {{ profile }}

# Build for web
[arg('profile', pattern='dev|release')]
build-web profile="dev":
    cargo build --target wasm32-unknown-emscripten --profile web-{{ profile }}
    @echo "Copying {{ package_name }}.data..."
    @cp ./target/wasm32-unknown-emscripten/web-{{ profile }}/deps/{{ package_name }}.data \
    ./target/wasm32-unknown-emscripten/web-{{ profile }}/{{ package_name }}.data
    @echo "Copying favicon.ico..."
    @cp ./static/favicon.ico ./target/wasm32-unknown-emscripten/web-\
        {{ profile}}/favicon.ico

# Serve the most recent web build
[arg('profile', pattern='dev|release')]
serve-web profile="dev":
    emrun index.html --serve_root ./target/wasm32-unknown-emscripten/web-{{ profile }}/ --port 8000

# Build and bundle specifically for Itch.io web player
bundle-itch: (build-web "release") (dist-guard "itch")
    #!/usr/bin/env bash
    set -euo pipefail
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
    cp ./static/favicon.ico ./dist/itch/build/favicon.ico

    echo "Zipping it all up..."
    pushd ./dist/itch
    zip {{ package_name }}-itch.zip build/*
    popd

    echo "Bundled for Itch.io!"

# Build and bundle for standalone web hosting
bundle-web: (build-web "release") (dist-guard "web")
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Moving build result..."
    cp ./target/wasm32-unknown-emscripten/web-release/index.html \
    ./dist/web/index.html
    cp ./target/wasm32-unknown-emscripten/web-release/{{ package_name }}.wasm \
        ./dist/web/{{ package_name }}.wasm
    cp ./target/wasm32-unknown-emscripten/web-release/{{ package_name }}.js \
        ./dist/web/{{ package_name }}.js
    cp ./target/wasm32-unknown-emscripten/web-release/{{ package_name }}.data \
        ./dist/web/{{ package_name }}.data
    cp ./static/favicon.ico ./dist/web/favicon.ico

    echo "Bundled for web!"

# Build and bundle an AppImage for Linux distribution
bundle-linux: (linux "release") (dist-guard "linux")
    #!/usr/bin/env bash
    set -euo pipefail

    mkdir ./dist/linux/build
    echo "> Moving build result..."
    cp ./target/x86_64-unknown-linux-gnu/release/{{ package_name }} \
        ./dist/linux/build

    echo "> Moving assets..."
    mkdir ./dist/linux/build/assets
    cp -r ./assets ./dist/linux/build/

    mkdir ./dist/linux/build/icons
    cp ./static/icon_256.png ./dist/linux/build/icons

    pushd ./dist/linux
    echo "> Creating bundle..."
    mkdir output.AppDir/
    touch output.AppDir/AppRun
    mkdir output.AppDir/usr
    mkdir output.AppDir/usr/bin
    mkdir output.AppDir/usr/share
    
    cat > output.AppDir/{{ package_name }}.desktop \
        << EOF
    [Desktop Entry]
    Version=1.0
    Name={{ package_name }}
    Exec=/usr/bin/{{ package_name }}
    Icon=/usr/share/icons/icon_256
    Terminal=false
    Type=Application
    Categories=Game
    EOF

    mv ./build/{{ package_name }} \
        output.AppDir/usr/bin
    mv ./build/assets \
        output.AppDir/usr/share
    mv ./build/icons \
        output.AppDir/usr/share

    echo "> Building AppImage"
    PKG=$(echo "{{ package_name }}" | sed "s/_/-/g")
    FULL_VERSION={{ package_version }}
    ARCH="x86_64"
    docker build . -t raylib_rs_env:bundle_appimage \
        --build-arg PACKAGE=$PKG \
        --build-arg FULL_VERSION=$FULL_VERSION \
        --build-arg ARCH=$ARCH \
        --file ../../docker/bundle/AppImage.Dockerfile
    id="$(docker create raylib_rs_env:bundle_appimage)"
    docker cp $id:/${PKG}_$FULL_VERSION-$ARCH.AppImage - \
            > ./${PKG}_$FULL_VERSION-$ARCH.AppImage

    popd
    echo "Bundled for Linux!"

# Build for Linux and bundle into *.deb binaries
bundle-deb: (linux "release") (dist-guard "deb")
    #!/usr/bin/env bash
    set -euo pipefail

    mkdir ./dist/deb/build
    echo "> Moving build result..."
    cp ./target/x86_64-unknown-linux-gnu/release/{{ package_name }} \
        ./dist/deb/build

    echo "> Moving assets..."
    mkdir ./dist/deb/build/assets
    cp -r ./assets ./dist/deb/build/

    mkdir ./dist/deb/build/icons
    cp ./static/icon_256.png ./dist/deb/build/icons

    pushd ./dist/deb
    echo "> Creating bundle..."
    mkdir -p output/DEBIAN
    mkdir -p output/usr/bin
    mkdir -p output/usr/share
    mkdir -p output/usr/share/applications
    mkdir -p output/usr/share/icons

    PKG=$(echo "{{ package_name }}" | sed "s/_/-/g")

    cat > output/DEBIAN/control << EOF
    Source: {{ package_name }}
    Section: games
    Priority: optional
    Package: $PKG
    Version: {{ package_version }}
    Architecture: amd64
    Maintainer: {{ package_authors }}
    Description: {{ package_description }}
    EOF

    cat > output/usr/share/applications/{{ package_name }}.desktop << EOF
    [Desktop Entry]
    Version=1.0
    Name={{ package_name }}
    Exec=/usr/bin/{{ package_name }}
    Icon=/usr/share/icons/icon_256
    Terminal=false
    Type=Application
    Categories=Game
    EOF

    mv ./build/{{ package_name }} \
        output/usr/bin
    mv ./build/assets output/usr/share
    mv ./build/icons output/usr/share

    echo "> Building with docker..."
    declare -a arr=("stable" "sid")
    for DEBIAN_DIST in "${arr[@]}"
    do
        echo "> Building $DEBIAN_DIST"
        FULL_VERSION={{ package_version }}+${DEBIAN_DIST}_x86_64
        docker build . -t raylib_rs_env:bundle_deb_${DEBIAN_DIST} \
            --build-arg DEBIAN_DIST=$DEBIAN_DIST \
            --build-arg PACKAGE=$PKG \
            --build-arg FULL_VERSION=$FULL_VERSION \
            --file ../../docker/bundle/Deb.Dockerfile
        id="$(docker create raylib_rs_env:bundle_deb_${DEBIAN_DIST})"
        docker cp $id:/${PKG}_$FULL_VERSION.deb - \
                > ./${PKG}_$FULL_VERSION.deb
    done

    popd
    echo "> Bundled for Debian!"

# Create bundles for Apple Intel and Apple Silicon computers
[parallel]
bundle-mac-all: bundle-mac-x86 bundle-mac-aarch64

# Bundle build product into a *.app for x86_64 Apple Intel computers
bundle-mac-x86: mac-guard (mac-x86 "release") (dist-guard "mac-x86_64")\
    (bundle-mac "x86_64")

# Bundle build product into a *.app for Apple Silicon computers
bundle-mac-aarch64: mac-guard (mac-arm "release") (dist-guard "mac-aarch64")\
    (bundle-mac "aarch64")

# Bundle Mac build product into *.app for either "x86_64" or "aarch64"
[arg('arch', pattern='x86_64|aarch64')]
[private]
bundle-mac arch:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "> Moving build result..."
    mkdir ./dist/mac-{{ arch }}/build
    cp ./target/{{ arch }}-apple-darwin/release/{{ package_name }} \
        ./dist/mac-{{ arch }}/build

    echo "> Moving assets..."
    mkdir ./dist/mac-{{ arch }}/build/assets
    cp -r ./assets ./dist/mac-{{ arch }}/build/
    mkdir ./dist/mac-{{ arch }}/build/icons
    cp ./static/apple_icons.icns ./dist/mac-{{ arch }}/build/icons
    
    pushd ./dist/mac-{{ arch }}
    otool -L ./build/{{ package_name }} | just ../../check-deps

    echo "> Assembling bundle..."
    mkdir {{ package_name }}_{{ arch }}.app/
    mkdir {{ package_name }}_{{ arch }}.app/Contents
    mkdir {{ package_name }}_{{ arch }}.app/Contents/MacOS
    mkdir {{ package_name }}_{{ arch }}.app/Contents/Resources
    mkdir {{ package_name }}_{{ arch }}.app/Contents/Resources/Data

    echo "> Clearing system cache..."
    /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister\
         -f {{ package_name }}_{{ arch }}.app

    echo "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
    <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
    <plist version=\"1.0\">
    <dict>
        <key>CFBundleName</key>
        <string>{{ package_name }}</string>
        <key>CFBundleDisplayName</key>
        <string>{{ package_name }}</string>
        <key>CFBundleExecutable</key>
        <string>{{ package_name }}</string>
        <key>CFBundleDevelopmentRegion</key>
        <string>en-GB</string>
        <key>CFBundleInfoDictionaryVersion</key>
        <string>6.0</string>
        <key>CFBundleVersion</key>
        <string>{{ package_version }}</string>
        <key>CFBundleIdentifier</key>
        <string>{{ package_identifier }}</string>
        <key>CFBundlePackageType</key>
        <string>APPL</string>
        <key>CFBundleIconFile</key>
        <string>icon.icns</string>
        <key>CSResourcesFileMapped</key>
        <true/>
    </dict>
    </plist>" > {{ package_name }}_{{ arch }}.app/Contents/Info.plist

    mv ./build/{{ package_name }} {{ package_name }}_{{ arch }}.app/Contents/MacOS
    mv ./build/assets {{ package_name }}_{{ arch }}.app/Contents/Resources
    mv ./build/icons/apple_icons.icns \
        {{ package_name }}_{{ arch }}.app/Contents/Resources/icon.icns

    echo "> Signing bundle..."
    codesign -f -s "SnailCreature" {{ package_name }}_{{ arch }}.app --deep
    popd

    echo "> Bundled for MacOS-{{ arch }}!"

# Check that the Mac target binary doesn't have unbundled dependencies
[private]
check-deps:
    #!/usr/bin/env python3
    import sys
    print('> Checking deps...')
    lines = sys.stdin.readlines()
    for line in lines:
        i = line.strip()
        ok = not i.startswith('/usr/lib/')\
            or not i.startswith('/System/Library/')
        if not ok:
            raise Exception(f'Non-OS dependency: {i}')

    print(''.join(lines))
    print('> Deps ok!')



# Install required dependencies for raylib/Rust development
setup: setup-emsdk setup-web setup-cross setup-platform

# Install system-specific dependencies
[private]
setup-platform:
    #!/usr/bin/env bash
    set -euo pipefail
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
[private]
setup-web:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "> Installing target wasm32-unknown-emscripten..."
    rustup target add wasm32-unknown-emscripten

# Setup cross and its dependencies
[private]
setup-cross:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "> Installing cross for cross-compilation..."
    cargo install cross --git https://github.com/cross-rs/cross

    echo "> Setting up docker image for Linux cross-compilation..."
    rustup target add x86_64-unknown-linux-gnu
    docker build --file ./docker/cross/CrossLinux.Dockerfile -t raylib_rs_env:linux .

    echo "> Setting up docker image for Windows cross-compilation..."
    docker build --file ./docker/cross/CrossWindows.Dockerfile -t raylib_rs_env:windows .


# Setup emscripten. Stolen lovingly from https://github.com/brettchalupa/sola-raylib
[private]
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

    Add that to your shell rc to persist it. Or don\'t: \`just build-web\` and
    \`just serve-web\` auto-source emsdk_env.sh from the default install path.

    Then:

        just build-web
        just serve-web                  # http://localhost:8000

    EOF

# Ensure current system is running MacOS
[private]
mac-guard:
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ ! {{ ostype }} == "darwin"* ]]; then
        echo "This is not a MacOS system."
        exit 1
    fi

# Ensure the dist directory for the given platform exists
[private]
dist-guard platform:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Ensuring ./dist exists..."
    if [ ! -d "./dist" ]; then
        mkdir ./dist
    fi

    echo "Ensuring ./dist/{{ platform }} exists and is clear..."
    if [ -d "./dist/{{ platform }}" ]; then
        rm -rf ./dist/{{ platform }}/*
    else
        mkdir ./dist/{{ platform }}
    fi
