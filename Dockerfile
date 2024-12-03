FROM rust:1.83-slim as builder

RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    cmake \
    clang \
    mold \
    libasound2-dev \
    libudev-dev \
    libx11-dev \
    libxcursor-dev \
    libxi-dev \
    libxrandr-dev \
    libxinerama-dev \
    libgl1-mesa-dev \
    libegl1-mesa-dev \
    libwayland-dev \
    libdbus-1-dev \
    libfreetype6-dev \
    libexpat1-dev \
    zlib1g-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

RUN cargo install just

WORKDIR /app

COPY . .

RUN rustup override set nightly
RUN rustup component add rustc-codegen-cranelift-preview

RUN just generate-release-folder-server


FROM rust:1.83-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    libasound2 \
    libudev1 \
    libx11-6 \
    libxcursor1 \
    libxi6 \
    libxrandr2 \
    libxinerama1 \
    libgl1-mesa-glx \
    libegl1-mesa \
    libwayland-client0 \
    libdbus-1-3 \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/release ./minecraft-rust-server-folder

EXPOSE 8000

CMD ["./minecraft-rust-server-folder/bin/minecraft-rust-server", "--world", "new_world" "--port", "8000"]
