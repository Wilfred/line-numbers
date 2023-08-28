default:
    @just --list

release:
    #!/bin/bash

    set -ex

    VERSION=$(cargo metadata --format-version=1 | jq -r '.packages | .[] | select(.name == "line-numbers") | .version')
    git tag $VERSION
    git push --tags

    cargo set-version --bump patch
