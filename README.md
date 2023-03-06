# The smart contracts for halotrade
The automated market-maker on [Aura](https://aura.network/) network.

## Contracts

|                  Name                    |                        Description                         |
| ---------------------------------------- | ---------------------------------------------------------- |
| [`halo_factory`](contracts/halo_factory) |                                                            |
| [`halo_pair`](contracts/halo_pair)       |                                                            |
| [`halo_router`](contracts/halo_router)   |                                                            |
| [`halo_token`](contracts/halo_token)     | CW20 (ERC20 equivalent) token implementation for LP tokens |

* halo_router

   Mainnet: `aura...`

   Testnet: `aura...`

* halo_factory

   Mainnet: `aura...`

   Testnet: `aura...`

* halo_pair

   Mainnet (CodeID): 

   Testnet (CodeID): 

* halo_token

   Mainnet (CodeID): 

   Testnet (CodeID): 

## Running these contracts

You will need Rust 1.66.0+ with wasm32-unknown-unknown target installed.

You can run tests on this on each contracts directory via :

```
cargo test
```

The contracts can be compiled using [beaker](https://github.com/osmosis-labs/beaker) 
```
beaker wasm build
```
with the optimizer is
```toml
optimizer_version = '0.12.9'
```

The optimized contracts are generated in the `artifacts/` directory.