# Raylib Template

A template repo for raylib-rs projects.

You will need raylib installed:

`brew install raylib`

And you will need Emscripten installed for web builds:

`brew install emscripten`

Or...

```bash
brew install just
just setup
```

[just](https://just.systems/man/en/)

## Inclusions

This template includes a handful of data structures and programming patterns
implemented for ease of use:

1. [Ecstasy](/engine/ecstasy/) - an ECS implementation
2. [Cacao](/engine/cacao/) - a very basic events system with observers and
   commands
3. [Quartermaster](/engine/quartermaster) - an asset manager with automatic
   memory management
1. [Ringo](/engine/ringo) - a pseudorandom number generator
2. [rlimgui](/engine/rlimgui) - DearIMGUI compatibility layer
3. [platform_compat](/engine/platform_compat) - Compatibility layer to ensure asset paths work across platforms

## Documentation

Check the [[./notes/index.md|notes]] folder for documentation on the various elements of this project
