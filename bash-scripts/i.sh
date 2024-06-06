#!/bin/bash

# setup command-line arguments
if [ -n "$1" ]; then
    RAFFLE_ID="$1"
else
    RAFFLE_ID=4249
fi

for e in ../artifacts/raffles.wasm; do 
echo $e;
starsd tx wasm i $RAFFLE_ID '{"name":"ohhnft-raffles","nois_proxy_addr":"stars1atcndw8yfrulzux6vg6wtw2c0u4y5wvy9423255h472f4x3gn8dq0v8j45","nois_proxy_coin":{"amount":"500000", "denom":"ustars"}, "raffle_fee": "0.1", "royalty_rate": "0.05", "fee_discounts": []}'  --from testnet --gas auto --admin stars1knfn5824464vg3hlk0rl0zrf0zmrydz3emnl0h --label "raffles" --fees 500000ustars --gas-adjustment 3 -y
done