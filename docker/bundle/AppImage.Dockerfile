FROM debian:stable

ARG PACKAGE
ARG FULL_VERSION
ARG ARCH=x86_64

RUN apt update && apt install -y wget file
RUN wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
RUN chmod +x linuxdeploy*.AppImage

COPY output.AppDir/ /output.AppDir

RUN ARCH=${ARCH} APPIMAGETOOL_APP_NAME=${PACKAGE}_${FULL_VERSION} \
    ./linuxdeploy-x86_64.AppImage --appimage-extract-and-run \
    --appdir output.AppDir --output appimage
