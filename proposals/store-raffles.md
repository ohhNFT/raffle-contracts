# Upload OhhNFT Raffles

We are proposing to upload the code for the OhhNFT Raffles contract, forked from the AtlasDAO Stargaze Raffles contract. This contract allows for the creation of raffles and the purchase of raffle tickets.

The source code is available at: https://github.com/OhhNFT/raffle-contracts

### Fee structure

There is a **5%** flat fee when creating a raffle. This fee is divided as follows:

- 10% royalties to the AtlasDAO team
- 10% used to buy & burn STARS
- 80% used to buy & burn STRDST

The fee will be reduced to **4%** if the creator owns any NFT of one of these collections:

- [Genesis First Press Owlies](https://www.stargaze.zone/m/stars1zvjnc08uy0zz43m0nlh9f5aetpa3amn6a034yqvmsgvzshk9cldsdhqjzg/tokens)
- [OhhNFT Presents: The Watchers](https://www.stargaze.zone/m/stars15jugk3w42q4a06df4p4tfqx5jnlrk2rg0c3sqj3ch0nhwyhj28ysnfvjf6/tokens)
- [OhhNFT Presents: The Fallen](https://www.stargaze.zone/m/stars1ma3nhs3ncsk7hurcend63ylxgc3awju2w9e8wrtedhhghhxf0mpq73p84x/tokens)

### Testnet deployment

- Code ID: `4249`
- Contract: `stars1k2pwhlmpl7elmf2kvpcjhjz79ushuqcvsv23dgp4v3uxlrxh0e6qnte4u0`

### SHA256 checksum

```
84764c3a19979f249d885f9390ca88e0bcbec4927b48859d7e31fd19afcf2faa  raffles.wasm
```

### Verify code

```
starsd  q gov proposal $id --output json \
| jq -r '.content.wasm_byte_code' \
| base64 -d \
| gzip -dc \
| sha256sum
```
