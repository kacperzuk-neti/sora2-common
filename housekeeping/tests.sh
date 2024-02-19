#!/bin/bash
set -e

test() {
    cargo test  --release --features runtime-benchmarks
}

if [ "$(type -t $1)" = "function" ]; then
    "$1"
else
    echo "Func '$1' is not exists in this workflow. Skipped."
fi