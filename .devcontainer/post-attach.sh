#!/bin/bash

mkdir -p /workspaces/iso10383/.cache/cargo
ln -sf /usr/local/cargo/bin /workspaces/iso10383/.cache/cargo/

cargo install -q cargo-semver-checks
cargo install -q release-plz

pushd /workspaces/iso10383 >/dev/null
pre-commit install >/dev/null
popd >/dev/null