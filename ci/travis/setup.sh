#!/bin/sh
# The purpose of this file is to install rustup, libsodium and sccache in the
# Travis CI environment. Outside this environment, you would probably not want
# to install them like this.

set -e -x

# Setup rustup
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

# Setup sccache
LATEST=$(cargo search -q sccache | grep sccache | cut -f2 -d"\"")
URL="https://github.com/mozilla/sccache/releases/download/${LATEST}/sccache-${LATEST}-x86_64-unknown-linux-musl.tar.gz"
curl -SsL $URL | tar xzvC /tmp
mv "/tmp/sccache-${LATEST}-x86_64-unknown-linux-musl/sccache" "${CARGO_HOME}/bin/sccache"
mkdir -p "$SCCACHE_DIR"

# Setup libsodium
mkdir -p $CARGO_HOME/cache/libsodium
cd $CARGO_HOME/cache/libsodium
if [ ! -e "$CARGO_HOME/cache/libsodium/.git" ]; then
  git clone "https://github.com/jedisct1/libsodium" .
else
  git pull
fi
./autogen.sh
./configure --prefix=$CARGO_HOME/ --disable-pie
make all install
cd $TRAVIS_BUILD_DIR
