#!/bin/bash
set -e -x

# Install rustup
curl -SsL https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain nightly

# Add cargo to path
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$CARGO_HOME/lib"
export SODIUM_STATIC="true"
export SODIUM_LIB_DIR="$CARGO_HOME/lib"

# Install libsodium
mkdir -p $CARGO_HOME/cache/libsodium
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
    "${PYBIN}/pip" install -U  setuptools setuptools-rust wheel
    "${PYBIN}/pip" wheel /io/ -w /io/dist/
done

# Bundle external shared libraries into the wheels
for whl in /io/dist/*.whl; do
    auditwheel repair "$whl" -w /io/dist/
done

# Install packages and test
for PYBIN in /opt/python/cp{27,35,36}*/bin/; do
    "${PYBIN}/pip" install hello-rust --no-index -f /io/dist/
done
