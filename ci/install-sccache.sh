#!/bin/sh

set -e

# check if sccache is already installed
if [ ! -f "${CARGO_HOME}/bin/sccache" ]; then
  LATEST=$(cargo search -q sccache | grep sccache | cut -f2 -d"\"")
  URL="https://github.com/mozilla/sccache/releases/download/${LATEST}/sccache-${LATEST}-x86_64-unknown-linux-musl.tar.gz"
  curl -SsL $URL | tar xzvC /tmp
  mv "/tmp/sccache-${LATEST}-x86_64-unknown-linux-musl/sccache" "${CARGO_HOME}/bin/sccache"
else
  echo 'Using cached sccache binary.'
fi

if [ ! -d "$SCCACHE_DIR" ]; then
  mkdir -p "$SCCACHE_DIR"
fi
