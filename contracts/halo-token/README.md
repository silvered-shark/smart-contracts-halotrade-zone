# halo-token
## Introduction

This repository contains the source code for a CW-20 token contract written in CosmWasm. The contract implements the CW-20 token standard on the AURA blockchain and provides basic functionality for transferring and managing tokens. The CW20 token contract provides a simple and easy way to create custom tokens without needing to write any code.


## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cosmos SDK](https://docs.cosmos.network/master/run-node/)
- [CosmWasm](https://docs.cosmwasm.com/0.16/getting-started/installation.html)

### Functionality

This CW-20 token contract implements all the required functionality for a [CW-20 token specification](https://github.com/CosmWasm/cw-plus/blob/main/packages/cw20/README.md).


### Installing:

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
beaker wasm store-code halo-token --signer-account signer --no-wasm-opt --network serenity
```

The result should be like this:
    
    ```bash
      Code stored successfully!! 🎉
    +
    ├── code_id: 1050
    └── instantiate_permission: –
    ```

3. Instantiate the contract

    ```bash
    beaker wasm instantiate halo-token --signer-account signer --raw '{"name": "Halo Token", "symbol": "HALO", "decimals": 6, "initial_balances": [{"address": "aura1x86wp9ys67hyltcy3wmy4g8wkp3x7u98pkd4pj", "amount": "3000000000000000"}], "mint": {"minter": "aura1txe6y425gk7ef8xp6r7ze4da09nvwfr2fhafjl", "cap": "2000000000000000"}}' --no-proposal-sync --network serenity
    ```

    The result should be like this:
    
    ```bash
    Contract instantiated successfully!! 🎉 
    +
    ├── label: default
    ├── contract_address: aura1qsa3zu5ahrlpqvaxll8thws66jywccs3qfg9f6qzemm0addfm6rs54gk26
    ├── code_id: 1050
    ├── creator: aura1txe6y425gk7ef8xp6r7ze4da09nvwfr2fhafjl
    └── admin: -
    ```

    Note: initial_balances.address is the address of the user who will receive the tokens. The address is the same as the address of the user who deployed the contract or not.

4. Query the contract

    4.1. Define the contract address
    ```bash
    CONTRACT=$(aurad query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')
    ```

    4.2. Update query message
    ```bash
    QUERY='{"balance":{"address":"aura1x86wp9ys67hyltcy3wmy4g8wkp3x7u98pkd4pj"}}'
    ```

    4.3. Query balance of user who received the tokens
    ```bash
    aurad query wasm contract-state smart $CONTRACT "$QUERY"  $NODE --output json
    ```

    4.1. Query cap
    ```bash
    QUERY='{"cap":{}}'
    aurad query wasm contract-state smart $CONTRACT "$QUERY"  $NODE --output json
    ```
    

5. Execute the contract

    5.1. Mint tokens
    ```bash
    beaker wasm execute halo-token --signer-account signer --raw '{"mint":{"recipient":"aura1x86wp9ys67hyltcy3wmy4g8wkp3x7u98pkd4pj", "amount":"1000000000000000"}}' --network serenity
    ```

    The result should be like this:
    
    ```bash
    Contract executed successfully!! 🎉
    +
    ├── label: default
    ├── contract_address: aura1qsa3zu5ahrlpqvaxll8thws66jywccs3qfg9f6qzemm0addfm6rs54gk26
    ```

### Testing the contract


To run the tests for the contract, run the following command:

```bash
    RUST_BACKTRACE=1 cargo unit-test
```

This will build the contract and run a series of tests to ensure that it functions correctly. The tests are defined in the ./tests directory.

To run the code coverage for the contract, run the following command:

```bash
    cargo tarpaulin --out Html   
```

You can receive tarpaulin-report.html that gives you the percentage of code coverage.

