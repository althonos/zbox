#!/bin/sh
# The purpose of this file is to install libsodium in
# the Travis CI environment. Outside this environment,
# you would probably not want to install it like this.

set -e

# check requested libsodium version
# if [ -z ${LIBSODIUM_VERSION+x} ]; then
#	VERSION=1.0.16;
# else
#	VERSION=$LIBSODIUM_VERSION;
# fi

URL=


# check if libsodium is already installed
if [ ! -f "$HOME/.local/lib/libsodium.a" ]; then
  curl -SsL "https://download.libsodium.org/libsodium/releases/LATEST.tar.gz" | tar xvz
  cd libsodium-*
  ./configure --prefix=$HOME/.local/ --disable-pie
  make clean all install
else
  echo 'Using cached directory.'
fi
