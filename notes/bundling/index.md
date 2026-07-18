# Bundling

Bundling a game for distribution is an important part of game creation. How else will all your friends get to experience your fun?
Included in this repo are bundling pipelines for all major platforms, including native Itch.io support,
although cross-compilation is not available for MacOS.

If you are unsure whether your system has been set up for bundling, run `just setup`.

The results of bundle operations will be output in the `dist` directory.

All bundle scripts use information from `Cargo.toml` to populate their outputs. Ensure you have the following populated:
- Name (`package.name`)
- Description (`package.description`)
- Authors (`package.authors`; Linux bundles require a Maintainer[^1], and only the first author will be listed)
- Version (`package.version`)

## Table of Contents
1. [[./linux.md|Linux]] - Creating deb and AppImage bundles
2. [[./mac.md|MacOS]] - Creating MacOS apps
3. [[./web.md|Web]] - Creating standalone websites and Itch.io bundles
4. [[./windows.md|Windows]] - Creating apps for Windows

---

## References
[^1] Binary packages: The maintainer of a package; `debian.org` [link](https://www.debian.org/doc/debian-policy/ch-binary.html#the-maintainer-of-a-package)
