FROM ubuntu:latest AS base
ENV DEBIAN_FRONTEND=noninteractive
SHELL ["bash", "-c"]

# https://github.com/electron-userland/electron-builder/blob/master/docker/wine/Dockerfile
# https://medium.com/better-programming/how-to-run-any-windows-cli-app-in-a-linux-docker-container-318cd49bdd25
RUN apt-get update && apt-get install -y wget gpg xvfb winbind
# Install wine
RUN source /etc/os-release && \
    dpkg --add-architecture i386 && \
    mkdir -pm755 /etc/apt/keyrings && \
    # wine
    # https://gitlab.winehq.org/wine/wine/-/wikis/Debian-Ubuntu#install-wine
    wget -O - https://dl.winehq.org/wine-builds/winehq.key | gpg --dearmor -o /etc/apt/keyrings/winehq-archive.key - && \
    wget -NP /etc/apt/sources.list.d/ https://dl.winehq.org/wine-builds/ubuntu/dists/$UBUNTU_CODENAME/winehq-$UBUNTU_CODENAME.sources && \
    apt-get -qq update && \
    apt-get -qq install -y --install-recommends winehq-stable && \
    # Wine 9.x+ removed wine64 as a standalone binary; wine on x86_64 is already 64-bit.
    # Symlink for backward compatibility with tools (e.g. electron-winstaller) that still check for wine64.
    ln -sf /usr/bin/wine /usr/bin/wine64 && \
    # powershell
    # https://learn.microsoft.com/en-us/powershell/scripting/install/install-ubuntu?view=powershell-7.4
    apt-get install -yq apt-transport-https software-properties-common && \
    wget -q https://packages.microsoft.com/config/ubuntu/22.04/packages-microsoft-prod.deb && \
    dpkg -i packages-microsoft-prod.deb && \
    rm packages-microsoft-prod.deb && \
    apt-get -qq update && \
    apt-get install -y powershell winetricks && \
    # clean
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set up wine
ENV WINEDEBUG=-all,err+all
ENV WINEDLLOVERRIDES=winemenubuilder.exe=d
ENV WINEARCH=win64

RUN winecfg
# Download wine-mono for DotNet compatibility
RUN wget -P /mono https://dl.winehq.org/wine/wine-mono/11.2.0/wine-mono-11.2.0-x86.msi
# Install winappcli
RUN wget -P /winappcli https://github.com/microsoft/winappCli/releases/download/v0.4.0/winappcli-x64.zip
RUN pushd /winappcli && \
unzip winappcli-x64.zip && \
popd

RUN wineboot --init 2>/tmp/wb.log; \
  if [ $? -ne 0 ]; then \
    cat /tmp/wb.log >&2; \
    grep -qE "host_page_mask|anon_mmap|qemu" /tmp/wb.log || \
      { echo "ERROR: wineboot failed on native x86_64" >&2; exit 1; }; \
    echo "NOTE: wineboot failed due to QEMU page-size emulation (expected on Apple Silicon/ARM cross-builds)" >&2; \
  fi; \
  rm -f /tmp/wb.log

# Install wine-mono
RUN msiexec /i /mono/wine-mono-11.2.0-x86.msi /qn
RUN rm -rf /mono/wine-mono-11.2.0-x86.msi

RUN xvfb-run winetricks dotnet9 -q

RUN wine cmd <<EOT
dotnet -h
dotnet add package Microsoft.WindowsAppSDK
dotnet add package Microsoft.Windows.SDK.BuildTools
EOT

# Check winappcli is working
RUN wine cmd <<EOT
/winappcli/winapp --help
EOT

FROM base AS bundle

ARG PACKAGE
ARG FULL_VERSION

RUN mkdir -p ./output/dist

COPY output/ ./output/dist
COPY Cargo.toml ./output/Cargo.toml

WORKDIR /output

RUN wine cmd <<EOT
/winappcli/winapp init
/winappcli/winapp pack ./dist --output ./${PACKAGE}_${FULL_VERSION}.msix \
--verbose
EOT

RUN ls -a
RUN ls dist -a
