FROM ghcr.io/cross-rs/x86_64-unknown-linux-gnu:edge

# Install CMake v4.4.0
RUN curl --retry 3 -sSfL "https://github.com/Kitware/CMake/releases/download/v4.4.0/cmake-4.4.0-linux-x86_64.sh" -o cmake.sh
RUN echo "6e7cdca8b054a3f6a5adcb1fa012e591e4c669bd744a009788681575aac96f50  cmake.sh" \
    | sha256sum --check --status
RUN sh cmake.sh --skip-license --prefix=/usr/local
RUN cmake --version

