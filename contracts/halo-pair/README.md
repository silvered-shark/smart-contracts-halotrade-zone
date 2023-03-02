# halo-pair
## Introduction

This is a CosmWasm smart contract for creating a pair of CW20 tokens on the AURA blockchain. This contract allows you to create and manage two CW20 tokens that can be traded against each other on a decentralized exchange (DEX) platform.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cosmos SDK](https://docs.cosmos.network/master/run-node/)
- [CosmWasm](https://docs.cosmwasm.com/0.16/getting-started/installation.html)

### Installing

- Clone the repository from: [Halo-swap repo](https://github.com/aura-nw/halo-swap)

```bash
git clone https://github.com/aura-nw/halo-swap.git
```

- Beaker tools:

```bash
cargo install -f beaker # `-f` flag for up-to-date version
```

### Build the contract

1. Build .wasm file stored in `target/wasm32-unknown-unknown/release/<CONTRACT_NAME>.wasm`
`--no-wasm-opt` is suitable for development, explained below

```bash
beaker wasm build --no-wasm-opt
```

### Deployment

1. Update Beaker.toml file

```bash
name = "halo-swap"
gas_price = '0.025uaura'
gas_adjustment = 1.3
account_prefix = 'aura'
derivation_path = '''m/44'/118'/0'/0/0'''

[networks.serenity]
chain_id = 'serenity-testnet-001'
network_variant = 'Shared'
grpc_endpoint = 'https://grpc.serenity.aura.network:9092'
rpc_endpoint = 'https://rpc.serenity.aura.network'

[accounts.signer]
mnemonic = 'around cushion believe vicious member trophy grit disease diagram nice only post nut beef mosquito thumb huge pelican disorder orchard response left phrase degree'

[wasm]
contract_dir = 'contracts'
optimizer_version = '0.12.9'
```

2. Store code on chain

Read .wasm in `target/wasm32-unknown-unknown/release/<CONTRACT_NAME>.wasm` due to `--no-wasm-opt` flag
use `--signer-account test1` which is predefined.
The list of all predefined accounts are here: https://github.com/osmosis-labs/LocalOsmosis#accounts
code-id` is stored in the beaker state, local by default

```bash
beaker wasm store-code halo-pair --signer-account signer --no-wasm-opt --network serenity
```

The result should be like this:
    
    ```bash
      Code stored successfully!! 🎉
    +
    ├── code_id: 1077
    └── instantiate_permission: –
    ```

3. Instantiate the contract

    ```
