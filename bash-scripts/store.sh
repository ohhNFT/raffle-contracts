#!/bin/bash
for e in ../artifacts/raffles.wasm; do 
echo $e;
starsd tx wasm store artifacts/raffles.wasm  --from testnet --gas auto --fees 10000000ustars --gas-adjustment 2 -y
done