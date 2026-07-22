FROM ubuntu:latest AS base
ENV DEBIAN_FRONTEND=noninteractive
SHELL ["bash", "-c"]

# https://github.com/electron-userland/electron-builder/blob/master/docker/wine/Dockerfile
# https://medium.com/better-programming/how-to-run-any-windows-cli-app-in-a-linux-docker-container-318cd49bdd25
RUN apt-get update && apt-get install -y \
wget \
gpg \
xvfb \
winbind \
unzip \
apt-transport-https \
software-properties-common
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
    wget -q https://packages.microsoft.com/config/ubuntu/22.04/packages-microsoft-prod.deb && \
    dpkg -i packages-microsoft-prod.deb && \
    rm packages-microsoft-prod.deb && \
    apt-get -qq update && \
    apt-get -y install powershell && \
    # clean
    apt-get clean 

# Download wine-mono for DotNet compatibility
RUN wget -P /usr/share/wine/mono https://dl.winehq.org/wine/wine-mono/11.2.0/wine-mono-11.2.0-x86.msi

# Install winappcli
RUN wget -P /winappcli https://github.com/microsoft/winappCli/releases/download/v0.5.0/winappcli-x64.zip
RUN pushd /winappcli && \
unzip winappcli-x64.zip && \
popd

# Download Powershell
# https://learn.microsoft.com/en-us/powershell/scripting/install/install-powershell-on-windows?view=powershell-7.4#install-the-zip-package
RUN wget -P /powershell https://github.com/PowerShell/PowerShell/releases/download/v7.6.3/PowerShell-7.6.3-win-x64.zip
RUN pushd /powershell && \
unzip PowerShell-7.6.3-win-x64.zip && \
popd

# Install donetsdk
RUN wget -P /dotnet https://builds.dotnet.microsoft.com/dotnet/Sdk/10.0.302/dotnet-sdk-10.0.302-win-x64.zip
RUN pushd /dotnet && \
unzip dotnet-sdk-10.0.302-win-x64.zip && \
popd

RUN adduser wineuser --quiet --disabled-login --home /home/wineuser --gecos ,,,
RUN chown -R wineuser:wineuser /winappcli \
&& chown -R wineuser:wineuser /powershell
WORKDIR /home/wineuser
USER wineuser

RUN echo "export XDG_RUNTIME_DIR=/run/user/$(id -u)" >> ~/.bashrc \
&& source ~/.bashrc

# Set up wine
ENV WINEDEBUG=-all,err+all,err-winediag,err-systray
ENV WINEDLLPATH="/powershell:/dotnet/10.0.302"
ENV WINEPATH="Z:\\powershell;Z:\\winappcli;Z:\\dotnet"
ENV WINEDLLOVERRIDES=winemenubuilder.exe=
ENV WINEARCH=win64

# ENV WINETRICKS_DOWNLOADER="wget"

RUN wineboot --init 2>/tmp/wb.log; \
  if [ $? -ne 0 ]; then \
    cat /tmp/wb.log >&2; \
    grep -qE "host_page_mask|anon_mmap|qemu" /tmp/wb.log || \
      { echo "ERROR: wineboot failed on native x86_64" >&2; exit 1; }; \
    echo "NOTE: wineboot failed due to QEMU page-size emulation (expected on Apple Silicon/ARM cross-builds)" >&2; \
  fi; \
  rm -f /tmp/wb.log

RUN wine reg add "HKCU\\Environment" /v PATH /t REG_EXPAND_SZ \
/d "Z:\\winappcli;%PATH%" /f \
&& wine reg add "HKCU\\Environment" /v PATH /t REG_EXPAND_SZ \
/d "Z:\\powershell;%PATH%" /f \
&& wine reg add "HKCU\\Environment" /v DOTNET_ROOT /t REG_EXPAND_SZ \
/d "Z:\\dotnet" /f \
&& wine reg add "HKCU\\Environment" /v PATH /t REG_EXPAND_SZ \
/d "%DOTNET_ROOT%;%PATH%" /f \
&& wine reg add "HKCU\\Environment" /v DOTNET_CLI_TELEMETRY_OPTOUT \
/d "true" /f \
&& wineboot --update

# pwsh -c Add-AppxPackage -RegisterByFamilyName -MainPackage Microsoft.DesktopAppInstaller_8wekyb3d8bbwe
RUN wine cmd <<EOT
pwsh -c $progressPreference = 'silentlyContinue'; \
Write-Host "Installing WinGet PowerShell module from PSGallery..."; \
Install-PackageProvider -Name NuGet -Force | Out-Null; \
Install-Module -Name Microsoft.WinGet.Client -Force -Repository PSGallery | \
Out-Null; \
Write-Host "Using Repair-WinGetPackageManager cmdlet to bootstrap WinGet..."; \
Repair-WinGetPackageManager -AllUsers; \
Write-Host "Done."
pwsh -c winget install Microsoft.Windows.Common-Controls
EOT

# Install wine-mono
RUN msiexec /i /usr/share/wine/mono/wine-mono-11.2.0-x86.msi /qn /nogui

RUN wine cmd <<EOT
dotnet help
dotnet dev-certs https --trust --quiet
EOT

RUN wineboot --update

# Check winappcli is working
RUN wine cmd <<EOT
winapp --help
EOT

RUN wineboot --shutdown --end-session
