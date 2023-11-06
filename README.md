# Give Credit

Give-credit is a soroban smart contract that allows you to retire carbon on stellar blockchain. The smart contract utilizes a host of other infrastructure to deliver a smooth user experience.

## Getting Started

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install the target wasm32-unknown-unknown

```bash
rustup target add wasm32-unknown-unknown
```

### Install the Soroban CLI

```bash
cargo install --locked --version 20.0.0-rc2 soroban-cli
```

### Setup Soroban Testnet Environment

First, we need to configure the soroban cli, so that we can deploy the contract to the soroban testnet.

```bash
soroban config network add --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

Then, configure an identity for the contract deployer.

```bash
./generate-id.sh
```

Fund the deployer account with testnet token:

```bash
 curl "https://friendbot.stellar.org/?addr=$(cat ./donation/.soroban/identity)"
```

## How to deploy the contract on the soroban testnet

### 1. Build the NFT contract

Go to nft-contract

```bash
cd ./nft-contract 
```

Then run the build with `make`.

> You will see that the Makefile build the nft contract, and output the contract .wasm to the [Target folder](nft-contract/target/wasm32-unknown-unknown/release)

Deploy the NFT contract. Run:

```bash
./deploy.sh
```

### Build and deploy the donation contract

Go to donation

```bash
cd ./donation 
```

Build the contract. Run:

```bash
soroban contract build
```

Deploy the contract. Run:

```bash
./deploy.sh
```

Initialize the contract. Run:

```bash
./initialize.sh
```
