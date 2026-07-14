FROM ghcr.io/cross-rs/x86_64-pc-windows-gnu:edge

#Install Raylib dependencies
RUN apt-get update
RUN apt install -y libclang-18-dev clang-18
RUN apt install build-essential git -y
RUN apt install -y \
    libasound2-dev \
    libx11-dev \
    libxrandr-dev \
    libxi-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libxcursor-dev \
    libxinerama-dev \
    libwayland-dev \
    libxkbcommon-dev

# Install CMake v4.4.0
RUN curl --retry 3 -sSfL "https://github.com/Kitware/CMake/releases/download/v4.4.0/cmake-4.4.0-linux-x86_64.sh" -o cmake.sh
RUN echo "6e7cdca8b054a3f6a5adcb1fa012e591e4c669bd744a009788681575aac96f50  cmake.sh" \
    | sha256sum --check --status
RUN sh cmake.sh --skip-license --prefix=/usr/local
RUN cmake --version

