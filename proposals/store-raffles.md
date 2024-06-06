# Upload OhhNFT Raffles

We are proposing to upload the code for the OhhNFT Raffles contract, forked from the AtlasDAO Stargaze Raffles contract. This contract allows for the creation of raffles and the purchase of raffle tickets.

The source code is available at: https://github.com/OhhNFT/raffle-contracts

### Fee structure

There is a **10%** fee upon creating a raffle. This fee is divided as follows:

- 10% royalties to the AtlasDAO team
- 10% used to buy & burn STARS
- 80% used to buy & burn STRDST

### SHA256 checksum

```
d7656019911745b97b54db86f6df2ef0a59ceaa12f70f358dd98fb8e29361720  raffles.wasm
```

### Verify code

```
starsd  q gov proposal $id --output json \
| jq -r '.content.wasm_byte_code' \
| base64 -d \
| gzip -dc \
| sha256sum
```
