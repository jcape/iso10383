#!/bin/bash

npm install -g @withgraphite/graphite-cli@stable

pushd /workspaces/iso10383 >/dev/null
pre-commit install >/dev/null
popd >/dev/null