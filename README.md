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

## Setting Up

1. Clone this repository
    ```sh
    git clone --depth=1 https://github.com/snailcreature/raylib_template.git <your project name>
    ```
1. Point it at your remote repository[^1]
    ```sh
    git remote set-url origin <your repo url>
    ```
1. Update `Cargo.toml` with your project details
4. Update `project_identifier` in the `justfile` (unless you are me)
5. Ensure `just` is installed and run the setup command
    ```sh
    just setup
    ```

## Inclusions

This template includes a handful of data structures and programming patterns
implemented for ease of use:

1. [Ecstasy](/engine/ecstasy/) - an ECS implementation (see also: [Ecstatic](https://github.com/snailcreature/ecstatic))
2. [Cacao](/engine/cacao/) - a very basic events system with observers and
   commands
3. [Quartermaster](/engine/quartermaster) - an asset manager with automatic
   memory management
1. [Ringo](/engine/ringo) - a pseudorandom number generator
2. [rlimgui](/engine/rlimgui) - DearIMGUI compatibility layer
3. [platform_compat](/engine/platform_compat) - Compatibility layer to ensure asset paths work across platforms

## Documentation

Check the [notes](/notes/index.md) folder for documentation on the various elements of this project

---

## References

[^1]: Answer to "How to properly fork a local git repository?"; `CodeWizard` - [https://stackoverflow.com/a/58113672](https://stackoverflow.com/a/58113672)
