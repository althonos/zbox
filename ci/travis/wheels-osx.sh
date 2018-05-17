#!/bin/bash
set -e +x

clean_project() {
    # Remove compiled files that might cause conflicts
    rm -rf /io/.cache /io/.eggs /io/build /io/*.egg-info
    find /io/ -name "__pycache__" -type d -print0 | xargs -0 rm -rf
    find /io/ -name "*.pyc" -type f -print0 | xargs -0 rm -rf
    find /io/ -name "*.so" -type f -print0 | xargs -0 rm -rf
}

# Install rustup
curl -SsL https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain nightly

# Add cargo to path
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"
export SCCACHE_DIR="$HOME/.cache/sccache"
export RUSTC_WRAPPER="sccache"
export SODIUM_STATIC="true"
export SODIUM_LIB_DIR="$CARGO_HOME/lib"
export LD_LIBRARY_PATH="$SODIUM_LIB_DIR:$LD_LIBRARY_PATH"

# Install libsodium
mkdir -p $CARGO_HOME/cache/libsodium
echo -e "\e[32m\e[1m Downloading\e[0m libsodium"
curl -SsL "https://download.libsodium.org/libsodium/releases/LATEST.tar.gz" | tar xzvC /tmp
cd /tmp/libsodium-*
./autogen.sh
./configure --prefix=$CARGO_HOME/ --disable-pie
make all install
cd /

# Install sccache
LATEST=$(cargo search -q sccache | grep sccache | cut -f2 -d"\"")
URL="https://github.com/mozilla/sccache/releases/download/${LATEST}/sccache-${LATEST}-x86_64-apple-darwin.tar.gz"
echo -e "\033[32m\033[1m Downloading\033[0m sccache v$LATEST"
curl -SsL $URL | tar xzvC /tmp
mv "/tmp/sccache-${LATEST}-x86_64-apple-darwin/sccache" "${CARGO_HOME}/bin/sccache"
mkdir -p "$SCCACHE_DIR"

# Compile wheels
echo -e "\033[32m\033[1m    Building\033[0m wheel for $(python --version)"
pip wheel . -w ./dist
