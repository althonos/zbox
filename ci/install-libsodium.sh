#!/bin/sh
# The purpose of this file is to install libsodium in
# the Travis CI environment. Outside this environment,
# you would probably not want to install it like this.

set -e

# check if libsodium is already installed
if [ ! -f "$CARGO_HOME/lib/libsodium.a" ]; then
  curl -SsL "https://download.libsodium.org/libsodium/releases/LATEST.tar.gz" | tar xvz
  cd libsodium-*
  ./configure --prefix=$CARGO_HOME/ --disable-pie
  make all install
else
  echo 'Using cached library.'
fi
