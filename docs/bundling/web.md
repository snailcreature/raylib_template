# Bundling for Web

Building and bundling a game for the web is vital, especially in game jams, where reviewers do not want to be downloading unknown software onto
their computers and you, the developer, don't want to be worrying about cross-platform compatibility. It's also very helpful for marketing a
complete project, allowing you to provide a quick demo for potential buyers to sample.

## Table of Contents
- [Bundling for Web](#bundling-for-web)
    - [Configuration](#configuration)
    - [Standalone](#standalone)
    - [Itch.io](#itchio)
    - [References](#references)

## Configuration

Raylib 6 has emscripten support from the get-go, however there are still certain configuration steps that needed to be made.
This project uses [sola-raylib](https://github.com/brettchalupa/sola-raylib) as its base, and `brettchalupa` has provided
some initial Rust configurations for deploying to the web[^1]. I used their recommended `.cargo/config.toml` set up, however made
some additions to ensure full compatibility after running into some issues. The full WASM configuration is:

```toml
[target.wasm32-unknown-emscripten]
rustflags = [
    "-C", "link-arg=-sMIN_WEBGL_VERSION=2",
    "-C", "link-arg=-sMAX_WEBGL_VERSION=2",
    "-C", "link-arg=-DPLATFORM_WEB",

    # Comment out these lines when building for release
    "-C", "link-arg=-Wall",
    "-C", "link-arg=-sGL_DEBUG",

    # raylib's web backend uses emscripten's bundled GLFW3 port.
    "-C", "link-arg=-sUSE_GLFW=3",
    # Let the wasm heap grow on demand.
    "-C", "link-arg=-sALLOW_MEMORY_GROWTH=1",
    "-C", "link-arg=-sINITIAL_MEMORY=512MB",
    "-C", "link-arg=-sMAXIMUM_MEMORY=2GB",
    # Deny GROWABLE_ARRAYBUFFERS to ensure compatibility with browsers other
    # than Safari
    "-C", "link-arg=-sGROWABLE_ARRAYBUFFERS=0",
    # raylib's audio backend (miniaudio) busy-waits via `emscripten_sleep(1)`
    # during WebAudio init. The first sound play aborts without ASYNCIFY.
    # The classic `while !rl.window_should_close()` loop also needs it.
    "-C", "link-arg=-sASYNCIFY=1",
    # raylib's image and audio loaders reach `fopen` through macros that
    # emcc's tree-shaker sometimes misses. Force the FS layer in.
    "-C", "link-arg=-sFORCE_FILESYSTEM=1",
    "-C", "link-arg=--preload-file", "-C", "link-arg=assets@/assets",
    # Match the wasm-EH ABI rustc 1.93+ emits. Without this you hit
    # `__cxa_find_matching_catch_*` link errors.
    "-C", "link-arg=-sSUPPORT_LONGJMP=wasm",
    # Recent emcc tree-shakes these aggressively. Without them, raylib's
    # GLFW glue and your shell's audio / heap access hit "Module.X is
    # undefined" at runtime.
    "-C", "link-arg=-sEXPORTED_RUNTIME_METHODS=['ccall','wasmMemory','HEAPU8','HEAP32','HEAPF32']",
]
```

The key additions are:
- `MIN_WEBGL_VERSION=2` and `MAX_WEBGL_VERSION=2` - Ensures the browser uses WebGL 2.0, as opposed to defaulting to WebGL 1.0
- `Wall` - Enables compile-time warnings for emscripten[^2]; Disable this on release builds
- `GL_DEBUG` - Enables debug information for WebGL[^2]; Disable this on release builds
- `GROWABLE_ARRAYBUFFERS=0` - Growable ArrayBuffers are a useful tool in WASM development, however the WebGL implementation on Firefox and
  Chrome are both very strict. Setting this flag to `0` ensures browsers other than Safari can run your game
- `INITIAL_MEMORY=512MB` and `MAXIMUM_MEMORY=2GB` - WASM can run into memory address issues when memory usage exceeds 2GB

`PLATFORM_WEB` is also set, although this is probably unnecessary.

Also included are the following general environment variables:

```toml
[env]
CFLAGS_wasm32_unknown_emscripten = "-fwasm-exceptions -sSUPPORT_LONGJMP=wasm"
CXXFLAGS_wasm32_unknown_emscripten = "-fwasm-exceptions -sSUPPORT_LONGJMP=wasm"
```

These are recommended inclusions from the `sola-raylib` book for when vendored libraries are included, such as `mlua`.

When targeting `emscripten`, the project `build.rs` script will automatically generate an `index.html` file, including the project name and
description. Using these, it also adds meta tags for card generation on social media. By default, this does not include the IndexDB mounting
needed for save data[^4] and should be added on a per-project basis.

## Standalone

Run `just bundle-web`. This will build the project against the `wasm32-unknown-emscripten` target and move the build result to `dist/web`.

```
dist
    |
    |- web
        |
        |- favicon.ico // Copy of static/favicon.ico
        |- index.html
        |- <project name>.data
        |- <project name>.js
        |- <project name>.wasm
```

These files can then be deployed on a web hosting service, such as Vercel, Cloudflare, or GitHub Pages, or added as a subdirectory of an
existing website.

## Itch.io

Run `just bundle-itch`. This will run the same steps as `bundle-web` and then compress the result into a `*.zip` archive that can be uploaded to
Itch.io for people to play in their browser of choice[^5]. The resulting bundle is output under `dist/itch`, and the uncompressed build artefacts
can be seen under `dist/itch/build`.

---

## References
[^1]: Building for the web (wasm + emscripten); `brettchalupa` - [link](https://github.com/brettchalupa/sola-raylib/blob/main/book/src/web.md)
[^2]: Working for Web (HTML5): Compile raylib game for web; `Laurentino` and `raysan5` - [link](https://github.com/raysan5/raylib/wiki/Working-for-Web-(HTML5)#5-compile-raylib-game-for-web)
[^3]: Emscripten Compiler Settings; `emscripten.org` - [link](https://emscripten.org/docs/tools_reference/settings_reference.html)
[^4]: Building for the web (wasm + emscripten): Save data; `brettchalupa` - [link](https://github.com/brettchalupa/sola-raylib/blob/main/book/src/web.md#save-data)
[^5]: Distributing Web games; `itch.io` - [link](https://itch.io/docs/itch/integrating/platforms/web.html)
