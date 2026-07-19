# Bundling for Linux

The popularity for gaming on Linux is growing, making it vital that it is supported natively as a platform, especially if you plan on
distributing your game directly or through Itch.io. This project provides bundling workflows for `x86_64` (`amd64`) systems as both `*.deb`
binary application bundles and AppImage bundles.

> [!IMPORTANT]
> Bundling for Linux currently relies on [cross](https://github.com/cross-rs/cross) and [Docker](https://docs.docker.com/get-started/get-docker/).
> It uses [custom Docker images](/docker) for both building and bundling, as `sola-raylib-sys` requires a newer version of `make` in order to be
> compiled.

## Table of Contents
- [Bundling for Linux](#bundling-for-linux)
    - [Deb Binary Application](#deb-binary-application)
        - [control](#control)
        - [Desktop Entry](#desktop-entry)
    - [AppImage](#appimage)
    - [References](#references)

## Deb Binary Application

Run `just bundle-deb`. This will produce two `*.deb` files under `dist/deb`, one for the current `stable` version of Debian (`trixie` at time of
writing) and one for the `unstable` version (lovingly named `sid`). The contents of those applications can be viewed under `dist/deb/output`.

```
\output
    |\DEBIAN
    |   \control
     \usr
        |\bin
        |   \<project_name> // The executable
         \share
            |\applications
            |   \<project_name>.desktop
            |\assets
            |   \<the contents of the assets folder...>
             \icons
                \icon_256.png // Copy of static/icon_256.png
```

> [!NOTE]
> The deb bundle script uses the [Deb.Dockerfile](/docker/bundle/Deb.Dockerfile) to convert the contents of `dist/deb/output` into the `*.deb`
> binaries.

### control

The `control` file contains all the information needed for the Debian bundler, in this case `dpkg-deb`. It will look something like this:

```
Source: <project_name>
Section: games
Priority: optional
Package: <project-name>
Version: <project version from Cargo.toml>
Architecture: amd64
Maintainer: <the first author listed in Cargo.toml>
Description: <project description from Cargo.toml>
```

This includes the mandatory fields described by the Debian documentation[^1] as well as the recommended Section and Priority fields. 
The build script pulls what information it can from the project's `Cargo.toml` file, so be sure that this reflects what you want to be included in the
bundle.

### Desktop Entry

The `*.desktop` file under `output/usr/share/applications` describes the binary's behaviour as a desktop application. It is similar to the
`control` file, but has a different specification[^3].

```desktop
[Desktop Entry]
Version=1.0
Name=<project_name>
Exec=/usr/bin/<project_name>
Icon=/usr/share/icons/icon_256
Terminal=false
Type=Application
Categories=Game
```

The Version field here refers to the version of the `*.desktop` specification[^2], not the version of the project, so should be left untouched.

## AppImage

Run `just bundle-linux`. This will produce a single `*.AppImage` file under `dist/linux`. Similar to the above `*.deb` bundle script, the
contents of the bundle can be viewed under `dist/linux/output.AppDir`.

```
output.AppDir
    |\AppRun
    |\<project_name>.desktop
     \usr
        |\bin
        |   \<project_name> // The executable
         \share
            |\assets
            |   \<contents of the assets folder...>
             \icons
                \icon_256.png // Copy of static/icon_256.png
```

> [!NOTE]
> The Linux bundle script uses the [AppImage.Dockerfile](/docker/bundle/AppImage.Dockerfile) to convert the contents of `dist/linux/output.Appdir`
> into the final `*.AppImage` executable.

The structure of the bundle is very similar to the Deb bundle structure, but without the `DEBIAN/control` file, the Desktop Entry file under
root, and the addition of the `AppRun` file. The AppRun file in `output.AppDir` is a placeholder: The Docker image uses `linuxdeploy`[^4] to
create the AppImage from the AppDir, which generates the AppRun executable automatically.

---

## References
[^1]: Control files and their fields: Debian binary package control files; `debian.org` - [https://www.debian.org/doc/debian-policy/ch-controlfields.html#debian-binary-package-control-files-debian-control]
[^2]: [https://github.com/flavienbwk/deb-package-tutorial/blob/main/mypackage_1.0_all/usr/share/applications/mypackage.desktop]
[^3]: Desktop entries; `ArchWiki` - [https://wiki.archlinux.org/title/Desktop_entries]
[^4]: linuxdeploy user guide; `appimage.org` - [https://docs.appimage.org/packaging-guide/from-source/linuxdeploy-user-guide.html]

Special thanks to [Dario Griffo and his article](https://dario.griffo.io/posts/ultimate-guide-debian-packaging/) on packaging `uv` for Debian.
