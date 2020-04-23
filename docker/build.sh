#!/usr/bin/env bash
#
# Build sandbox images
#

set -ex

for language in c cpp rust go; do
    docker build                \
        --rm                    \
        --force-rm              \
        --tag sandbox-$language \
        $language
done
