#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
OUTPUT="./programs/.bin"
# go to parent folder
cd $(dirname ${SCRIPT_DIR})

cargo test-sbf