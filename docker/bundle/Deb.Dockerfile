ARG DEBIAN_DIST=bookworm
FROM debian:${DEBIAN_DIST}

ARG DEBIAN_DIST
ARG PACKAGE
ARG FULL_VERSION

RUN apt update
RUN mkdir -p /output/usr/bin
RUN mkdir -p /output/DEBIAN

COPY output/DEBIAN/control /output/DEBIAN/
COPY output/usr/bin /output/usr/bin

RUN dpkg-deb --build /output /${PACKAGE}_${FULL_VERSION}.deb
