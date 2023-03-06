# The factory contract for Haloswap

The factory contract will handle the information related to pairs.

## InstantiateMsg
We must provide the source code id of `halo_pair` contract and `halo-token` contract for `halo-factory` contract.
```javascript
{
    "pair_code_id": 123,
    "token_code_id": 123
}
```

## ExecuteMsg

### UpdateConfig
```javascript
{
    "update_config": {
        "owner": "aura...",
        "token_code_id": 321,
        "pair_code_id": 321
    }
}
```

### CreatePair
The parameters in `requirements` include the whitelisted users who can provide liquidity for the first time when pair is empty and the minimum amount of assets that users must provide in the first time.
```javascript
{
    "create_pair": {
        "asset_infos": [
            {
                "token": {
                    "contract_addr": "aura..."
                }
            },
            {
                "native_token": {
                    "denom": "uaura"
                }
            }
        ],
        "requirements": {
            "whitelist": [
                "aura...",
                "aura..."
            ],
            "first_asset_minimum": 10000,
            "second_asset_minimum": 20000
        }
    }
}
```

### AddNativeTokenDecimals
Before can be added to any pair, a native token must be specified its decimals.
```javascript
{
    "add_native_token_decimals": {
        "denom": "uaura",
        "decimals": 6,
    }
}
```

### MigratePair
```javascript
{
    "migrate_pair" {
        "contract": "aura...",
        "code_id": 321
    }
}
```

## QueryMsg
### Config
```javascript
{
    "config": {}
}
```

### Pair
```javascript
{
    "pair": {
        "asset_infos": [
            {
                "token": {
                    "contract_addr": "aura..."
                }
            },
            {
                "native_token": {
                    "denom": "uaura"
                }
            }
        ]
    }
}
```
    
### Pairs
```javascript
{
    "pairs": { }
}
```

### NativeTokenDecimals
```javascript
{
    "native_token_decimals" {
        "denom": "uaura",
    },
}
    