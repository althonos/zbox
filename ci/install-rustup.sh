#!/bin/sh

set -e

curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
