#!/bin/sh

set -e

# check if sccache is already installed
if [ ! -f "$CARGO_HOME/bin/sccache" ]; then
  RUSTC_WRAPPER="" cargo install sccache --debug
else
  echo 'Using cached sccache binary.'
fi
