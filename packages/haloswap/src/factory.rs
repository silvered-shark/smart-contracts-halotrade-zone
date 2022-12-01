use cosmwasm_schema::cw_serde;

use crate::asset::{AssetInfo, PairInfo};

#[cw_serde]
pub struct InstantiateMsg {
    /// Pair contract code ID, which is used to
    pub pair_code_id: u64,
    pub token_code_id: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// UpdateConfig update relevant code IDs
    UpdateConfig {
        owner: Option<String>,
        token_code_id: Option<u64>,
        pair_code_id: Option<u64>,
    },
    /// CreatePair instantiates pair contract
    CreatePair {
        /// Asset infos
        asset_infos: [AssetInfo; 2],
    },
    AddNativeTokenDecimals {
        denom: String,
        decimals: u8,
    },
    MigratePair {
        contract: String,
        code_id: Option<u64>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Pair {
        asset_infos: [AssetInfo; 2],
    },
    Pairs {
        start_after: Option<[AssetInfo; 2]>,
        limit: Option<u32>,
    },
    NativeTokenDecimals {
        denom: String,
    },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct ConfigResponse {
    pub owner: String,
    pub pair_code_id: u64,
    pub token_code_id: u64,
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}

// We define a custom struct for each query response
#[cw_serde]
pub struct PairsResponse {
    pub pairs: Vec<PairInfo>,
}

#[cw_serde]
pub struct NativeTokenDecimalsResponse {
    pub decimals: u8,
}
