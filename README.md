# Give Credit

Give-credit is a soroban smart contract that allows you to retire carbon on stellar blockchain. The smart contract utilizes a host of other infrastructure to deliver a smooth user experience.

## How to deploy the contract on the soroban testnet

### 1. Build the NFT contract

Go to nft-contract
```
cd ./nft-contract 
```

Then run the build with `make`.

> You will see that the Makefile build the nft contract, and output the contract .wasm to the [Target folder](nft-contract/target/wasm32-unknown-unknown/release)
