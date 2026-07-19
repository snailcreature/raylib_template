# Bundling for MacOS

MacOS is not a common target for games. However, if you have access to a Mac (or splash out on GitHub Actions), you can make a small community
of Apple gamers very happy.

> [!IMPORTANT]
> `cross` does support cross-compilation to Apple devices, however due to licensing, it's a situation of "bring your own SDK"[^1]. Additionally,
> I am unsure if the additional tools for checking dependencies and signing the `*.app` bundle are available on the Ubuntu Docker image that `cross` uses
> as a base for their `darwin` targets.

## Code Signing

The bundle script uses the `codesign` command to sign the bundle to meet the minimum security requirements for Apple apps. You will need to
create a code signing certificate in your Keychain on your Mac in order to facilitate this.

1. Open Keychain Access
2. Open the Certificate Assistant (`Keychain Access > Certificate Assistant > Create a certificate...`, or search "create a certificate" in the Help menu)
3. Enter a name for the certificate
4. Ensure the Identity Type is set to Self-Signed Root
5. Set the Certificate Type to Code Signing
6. Click Create

You now have a code signing certificate on your Keychain. Update the `codesign_keychain_ident` variable in the `justfile` to match the name of
the certificate you just created.

## Bundling

The app creation process is perhaps second only to bundling for web in terms of ease: Apple apps are actually just UI trickery that displays
directories with certain formats and contents as apps rather than just folders[^2].

Run `just bundle-mac-x86` or `just bundle-mac-aarch64` (or `just bundle-mac-all` to build both in parallel). This will create separate folders
under `dist` for your target architecture: `dist/mac-x86` and `dist/mac-aarch64`. `mac-x86` targets older hardware that uses the Intel chipset,
whilst `mac-aarch64` targets the newer Apple Silicon devices. Both have the same package structure. You can open the bundle in your code
editor like any other directory, or find it in Finder and open it by right-clicking the app and clicking Show Package Contents.

```
<project_name>_<architecture>.app
    \Contents
        |\Info.plist
        |\MacOS
        |   \<project_name> // The executable
        |\Resources
        |   |\icon.icns // Copy of static/apple_icons.icns
        |   |\Data
        |    \assets
        |       \<contents of the assets folder...>
         \_CodeSignature
            \CodeResources
```

### Info.plist

This is where the magic happens. Info.plist is an XML file that tells the operating system that this is an app, what kind of app it is, and
where it can find the various resources it needs. The bundler script uses information from the project `Cargo.toml` where it can, however you
will need to update the `package_identifier` variable in the `justfile` with your own unique identifier.

```xml
<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
    <plist version=\"1.0\">
    <dict>
        <key>CFBundleName</key>
        <string><package_name></string>
        <key>CFBundleDisplayName</key>
        <string><package_name></string>
        <key>CFBundleExecutable</key>
        <string><package_name></string>
        <key>CFBundleDevelopmentRegion</key>
        <string>en-GB</string>                  // Change this to match your language code
        <key>CFBundleInfoDictionaryVersion</key>
        <string>6.0</string>
        <key>CFBundleVersion</key>
        <string><package version></string>
        <key>CFBundleIdentifier</key>
        <string><package identifier></string>
        <key>CFBundlePackageType</key>
        <string>APPL</string>
        <key>CFBundleIconFile</key>
        <string>icon.icns</string>
        <key>CSResourcesFileMapped</key>
        <true/>
    </dict>
</plist>
```

For full descriptions of each of these tags, please check the Apple Developer documentation[^3] and the Developer archive[^4].

---

## References
[^1]: `cross-toolchains`: Apple Targets; `cross-rs` - [https://github.com/cross-rs/cross-toolchains#apple-targets]
[^2]: Working on macOS: Bundle your app in an Application; `FilippoPaganelli` and `raysan5` - [https://github.com/raysan5/raylib/wiki/Working-on-macOS#bundle-your-app-in-an-application]
[^3]: Bundle Resources; `Apple` - [https://developer.apple.com/documentation/bundleresources]
[^4]: Core Foundation Keys; `Apple` - [https://developer.apple.com/library/archive/documentation/General/Reference/InfoPlistKeyReference/Articles/CoreFoundationKeys.html]
