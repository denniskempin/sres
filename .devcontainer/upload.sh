#!/usr/bin/env bash

cd "$(dirname "${BASH_SOURCE[0]}")" || exit 1

docker build --platform linux/amd64 -t ghcr.io/denniskempin/res:amd64-latest .
docker build --platform linux/arm64 -t ghcr.io/denniskempin/res:arm64-latest .

docker push ghcr.io/denniskempin/res:amd64-latest
docker push ghcr.io/denniskempin/res:arm64-latest

docker manifest create ghcr.io/denniskempin/res:latest \
    --amend ghcr.io/denniskempin/res:amd64-latest \
    --amend ghcr.io/denniskempin/res:arm64-latest

docker manifest push ghcr.io/denniskempin/res:latest
