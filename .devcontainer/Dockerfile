FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# Debugging/profiling tools and system libraries for solver crates (z3, good_lp's coin_cbc backend)
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends \
        lldb \
        python3-lldb-14 \
        linux-perf \
        libz3-dev \
        clang \
        libclang-dev \
        coinor-libcbc-dev \
        cmake \
        pkg-config \
        python3 \
    && rm -rf /var/lib/apt/lists/* \
    # Debian's lldb symlink points at an empty python3 dir; point it at the real 3.11 package
    && ln -sfn /usr/lib/llvm-14/lib/python3.11/dist-packages/lldb /usr/lib/python3/dist-packages/lldb

# Install cargo tools as the container user so they land in its cargo home
USER vscode
RUN cargo install --locked aoc-cli bacon hyperfine flamegraph \
    && rm -rf /usr/local/cargo/registry
