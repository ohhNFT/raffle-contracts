#!/bin/bash

# setup command-line arguments
if [ -n "$1" ]; then
    RAFFLE_CONTRACT="$1"
else
    RAFFLE_CONTRACT=
fi
    
# compute expected results
resr=$(st q wasm contract $RAFFLE_CONTRACT)

# get code id
code_idr=$(echo $resr | jq -r '.contract_info.code_id')

# download binaries from network
st q wasm code $code_idr raffle-code.wasm

# verify codehash
sha256sum raffle-code.wasm
# d7656019911745b97b54db86f6df2ef0a59ceaa12f70f358dd98fb8e29361720  raffle-code.wasm


