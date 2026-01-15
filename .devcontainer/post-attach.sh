#!/bin/bash

mkdir -p /workspaces/iso10383/.cache/cargo
ln -sf /usr/local/cargo/bin /workspaces/iso10383/.cache/cargo/

rustup toolchain install stable
rustup component add --toolchain stable rustfmt
rustup toolchain install nightly
rustup component add --toolchain nightly rustfmt

cargo binstall -q -y --force prek
cargo binstall -q -y --force action-validator
cargo binstall -q -y --force cargo-deny
cargo binstall -q -y --force cargo-nextest

pushd /workspaces/iso10383 >/dev/null
prek install >/dev/null
popd >/dev/null
