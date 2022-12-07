/// We will set up a mock environment for testing
/// Then instantiate all the contracts we need
use cosmwasm_std::{Addr, Coin, Uint128, Empty};
use cw20::{Cw20Coin, MinterResponse};
use cw_multi_test::{App, AppBuilder, ContractWrapper, Contract, Executor};
use halo_token::contract::{execute as halo_token_execute, instantiate as halo_token_instantiate, query as halo_token_query};
use halo_pair::contract::{execute as halo_pair_execute, instantiate as halo_pair_instantiate, query as halo_pair_query, reply as halo_pair_reply};
use halo_factory::contract::{execute as halo_factory_execute, instantiate as halo_factory_instantiate, query as halo_factory_query, reply as halo_factory_reply};
use halo_router::contract::{execute as halo_router_execute, instantiate as halo_router_instantiate, query as halo_router_query};

// use haloswap::asset::AssetInfo;
use haloswap::token::InstantiateMsg as HaloTokenInstantiateMsg;
// use haloswap::pair::InstantiateMsg as HaloPairInstantiateMsg;
use haloswap::factory::InstantiateMsg as HaloFactoryInstantiateMsg;
use haloswap::router::InstantiateMsg as HaloRouterInstantiateMsg;

pub const USER: &str = "aura1fqj2redmssckrdeekhkcvd2kzp9f4nks4fctrt";
pub const ADMIN: &str = "aura1uh24g2lc8hvvkaaf7awz25lrh5fptthu2dhq0n";
pub const NATIVE_DENOM: &str = "uaura";

pub const TOKEN_INITIAL_BALANCE: u128 = 1000000000000u128;

fn mock_app() -> App {
    AppBuilder::new().build(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(ADMIN),
                vec![Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: Uint128::new(1000000000000u128.into()),
                }],
            )
            .unwrap();
    })
}

fn halo_token_contract_template() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        halo_token_execute,
        halo_token_instantiate,
        halo_token_query,
    );
    Box::new(contract)
}

fn halo_pair_contract_template() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        halo_pair_execute,
        halo_pair_instantiate,
        halo_pair_query,
    ).with_reply(halo_pair_reply);
    Box::new(contract)
}

fn halo_factory_contract_template() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        halo_factory_execute,
        halo_factory_instantiate,
        halo_factory_query,
    ).with_reply(halo_factory_reply);
    Box::new(contract)
}

fn halo_router_contract_template() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        halo_router_execute,
        halo_router_instantiate,
        halo_router_query,
    );
    Box::new(contract)
}

/// function to instantiate all contracts
/// @note the address of contract pair_A_and_B & the address of LP token will be retrieved when user add new pair to factory
/// @return app: App - the app instance
/// @return token_A: halo_token - the address of token A
/// @return token_B: halo_token - the address of token B
/// @return swap_factory: halo_factory - the address of swap factory contract
/// @return swap_router: halo_router - the address of swap router contract
pub fn instantiate_contracts() -> (App, String, String, String, String) {
    // Create a new app instance
    let mut app = mock_app();

    // store the code of contract_template an get code ID
    let halo_token_id = app.store_code(halo_token_contract_template());
    let halo_pair_id = app.store_code(halo_pair_contract_template());
    let halo_factory_id = app.store_code(halo_factory_contract_template());
    let halo_router_id = app.store_code(halo_router_contract_template());

    // create instantiate message for token_A
    let token_a_instantiate_msg = HaloTokenInstantiateMsg {
        name: "Token A".to_string(),
        symbol: "TKA".to_string(),
        decimals: 6,
        initial_balances: [
            Cw20Coin {
                address: ADMIN.to_string(),
                amount: Uint128::new(TOKEN_INITIAL_BALANCE),
            },
        ].to_vec(),
        mint: Some(MinterResponse {
            minter: ADMIN.to_string(),
            cap: Some(Uint128::new(TOKEN_INITIAL_BALANCE)),
        }),
    };

    // instantiate token_A
    let token_a_contract_addr = app
        .instantiate_contract(
            halo_token_id,
            Addr::unchecked(ADMIN),
            &token_a_instantiate_msg,
            &[],
            "test instantiate token A",
            None,
        )
        .unwrap();

    // create instantiate message for token_B
    let token_b_instantiate_msg = HaloTokenInstantiateMsg {
        name: "Token B".to_string(),
        symbol: "TKB".to_string(),
        decimals: 6,
        initial_balances: [
            Cw20Coin {
                address: ADMIN.to_string(),
                amount: Uint128::new(TOKEN_INITIAL_BALANCE),
            },
        ].to_vec(),
        mint: Some(MinterResponse {
            minter: ADMIN.to_string(),
            cap: Some(Uint128::new(TOKEN_INITIAL_BALANCE)),
        }),
    };

    // instantiate token_B
    let token_b_contract_addr = app
        .instantiate_contract(
            halo_token_id,
            Addr::unchecked(ADMIN),
            &token_b_instantiate_msg,
            &[],
            "test instantiate token B",
            None,
        )
        .unwrap();
    
    // // create instantiate message for pair_A_and_B
    // let pair_a_and_b_instantiate_msg = HaloPairInstantiateMsg {
    //     asset_infos: [
    //         AssetInfo::Token {
    //             contract_addr: token_a_contract_addr.to_string(),
    //         },
    //         AssetInfo::Token {
    //             contract_addr: token_b_contract_addr.to_string(),
    //         },
    //     ],
    //     token_code_id: halo_token_id,
    //     asset_decimals: [6u8, 6u8],
    // };

    // // instantiate pair_A_and_B
    // let pair_a_and_b_contract_addr = app
    //     .instantiate_contract(
    //         halo_pair_id,
    //         Addr::unchecked(ADMIN),
    //         &pair_a_and_b_instantiate_msg,
    //         &[],
    //         "test instantiate pair A and B",
    //         None,
    //     )
    //     .unwrap();

    // create instantiate message for swap_factory
    let swap_factory_instantiate_msg = HaloFactoryInstantiateMsg {
        pair_code_id: halo_pair_id,
        token_code_id: halo_token_id,
    };

    // instantiate swap_factory
    let swap_factory_contract_addr = app
        .instantiate_contract(
            halo_factory_id,
            Addr::unchecked(ADMIN),
            &swap_factory_instantiate_msg,
            &[],
            "test instantiate swap factory",
            None,
        )
        .unwrap();

    // create instantiate message for swap_router
    let swap_router_instantiate_msg = HaloRouterInstantiateMsg {
        halo_factory: swap_factory_contract_addr.to_string(),
    };

    // instantiate swap_router
    let swap_router_contract_addr = app
        .instantiate_contract(
            halo_router_id,
            Addr::unchecked(ADMIN),
            &swap_router_instantiate_msg,
            &[],
            "test instantiate swap router",
            None,
        )
        .unwrap();

    // return the app instance and the addresses of all contracts
    (
        app,
        token_a_contract_addr.to_string(),
        token_b_contract_addr.to_string(),
        swap_factory_contract_addr.to_string(),
        swap_router_contract_addr.to_string(),
    )
}