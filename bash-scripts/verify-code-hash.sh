#!/bin/bash

# setup command-line arguments
if [ -n "$1" ]; then
    RAFFLE_ID="$1"
else
    RAFFLE_ID=3650
fi
    
# compute expected results
sha256sum target/wasm32-unknown-unknown/release/raffles.wasm

# download binaries from network
st q wasm code $RAFFLE_ID raffle-code.wasm

# compute download binary checksums 
sha256sum raffle-code.wasm 

# d7656019911745b97b54db86f6df2ef0a59ceaa12f70f358dd98fb8e29361720  raffle-code.wasm