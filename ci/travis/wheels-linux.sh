#!/bin/bash
set -e -x

clean_project() {
    # Remove compiled files that might cause conflicts
    rm -rf /io/.cache /io/.eggs /io/build /io/*.egg-info
    find /io/ -name "__pycache__" -type d -print0 | xargs -0 rm -rf
    find /io/ -name "*.pyc" -type f -print0 | xargs -0 rm -rf
    find /io/ -name "*.so" -type f -print0 | xargs -0 rm -rf
}

# Add cargo to path
export CARGO_HOME="/usr"
export RUSTUP_HOME="/usr"
export SCCACHE_DIR="/var/cache/sccache"
export RUSTC_WRAPPER="sccache"
export SODIUM_STATIC="true"
export SODIUM_LIB_DIR="$CARGO_HOME/lib"

# Install rustup
curl -SsL https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain nightly

# Install sccache
LATEST=$(cargo search -q sccache | grep sccache | cut -f2 -d"\"")
URL="https://github.com/mozilla/sccache/releases/download/${LATEST}/sccache-${LATEST}-x86_64-unknown-linux-musl.tar.gz"
echo -e "\e[32m\e[1m Downloading\e[0m sccache v$LATEST"
curl -SsL $URL | tar xzvC /tmp
mv "/tmp/sccache-${LATEST}-x86_64-unknown-linux-musl/sccache" "${CARGO_HOME}/bin/sccache"
mkdir -p "$SCCACHE_DIR"

# Install libsodium
mkdir -p $CARGO_HOME/cache/libsodium
echo -e "\e[32m\e[1m Downloading\e[0m libsodium"
curl -SsL "https://download.libsodium.org/libsodium/releases/LATEST.tar.gz" | tar xzvC /tmp
cd /tmp/libsodium-*
./autogen.sh
./configure --prefix=$CARGO_HOME/ --disable-pie
make all install
cd /

# Compile wheels
for PYBIN in /opt/python/cp{27,35,36}*/bin; do
    export PYTHON_SYS_EXECUTABLE="$PYBIN/python"
    export PYTHON_LIB=$(${PYBIN}/python -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))")
    export LIBRARY_PATH="$LIBRARY_PATH:$PYTHON_LIB"
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$PYTHON_LIB"
    echo -e "\e[32m\e[1m    Building\e[0m wheel for $($PYBIN/python --version)"
    "${PYBIN}/pip" install -U  setuptools setuptools-rust wheel
    "${PYBIN}/pip" wheel /io/ -w /io/dist/
done

# Bundle external shared libraries into the wheels
for whl in /io/dist/*linux*.whl; do
    echo -e "\e[32m\e[1m    Auditing\e[0m wheel $(basename $whl)"
    auditwheel repair "$whl" -w /io/dist/
    rm $whl
done
