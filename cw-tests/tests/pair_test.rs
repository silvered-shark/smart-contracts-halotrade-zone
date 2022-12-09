use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::Executor;
use tests::environment::{ADMIN, instantiate_contracts};
use haloswap::factory::{ExecuteMsg as FactoryExecuteMsg};
use haloswap::asset::{AssetInfo, PairInfo};
use haloswap::pair::{ExecuteMsg as PairExecuteMsg, QueryMsg as PairQueryMsg};

// test to add liquidity to a pair of cw20 tokens
mod add_liquidity_to_cw20_and_cw20 {
    use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, BalanceResponse};
    use haloswap::asset::Asset;

    use super::*;

    // cannot provide liquidity if first asset wrong
    #[test]
    #[should_panic(expected = "Wrong asset info is given")]
    fn cannot_provide_liquidity_if_first_asset_wrong() {
        // instantiate contracts
        let (mut app,
            token_a_contract_addr,
            token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            code_ids
        ) = instantiate_contracts();

        // create message to create new pair
        let msg = FactoryExecuteMsg::CreatePair {
            asset_infos: [
                AssetInfo::Token {
                    contract_addr: token_a_contract_addr.clone()
                },
                AssetInfo::Token {
                    contract_addr: token_b_contract_addr.clone()
                },
            ],
        };

        // execute create pair message on factory contract
        let res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        ).unwrap();

        // define variables to store the contract addresses of the new pair and lp token
        let mut pair_contract_addr: String = "".to_string();
        let mut lp_token_contract_addr: String = "".to_string();

        // parse events from response
        let events = res.events;

        // loop through events and check the attributes of each event
        for event in events {
            // if the ty of the event is "instantiate"
            if event.ty == "instantiate" {
                // declare a variable to store the contract address temporarily
                let mut temp_contract_addr: String = "".to_string();
                // declare a variable to store the code id temporarily
                let mut temp_code_id: String = "".to_string();

                // loop through the attributes of the event
                for attribute in event.attributes {
                    // if the key of the attribute is "_contract_addr" or "code_id"
                    if attribute.key == "_contract_addr" {
                        // set the value of the attribute to the pair_contract_addr variable
                        temp_contract_addr = attribute.value;
                    } else if attribute.key == "code_id" {
                        // set the value of the attribute to the code_id variable
                        temp_code_id = attribute.value;
                    }
                }

                // if the code_id is equal to the code_id of the pair contract
                if temp_code_id == code_ids.halo_pair_code_id.to_string() {
                    // set the value of the pair_contract_addr variable
                    pair_contract_addr = temp_contract_addr;
                } else if temp_code_id == code_ids.halo_token_code_id.to_string() {
                    // set the value of the lp_token_contract_addr variable
                    lp_token_contract_addr = temp_contract_addr;
                }
            }
        }

        // prepare the query pair message
        let msg = PairQueryMsg::Pair {};

        // query the pair info of pair contract
        let pair_info: PairInfo = app.wrap().query_wasm_smart(pair_contract_addr.clone(), &msg).unwrap();

        // the asset_infos of the pair info should include the token_a_contract_addr and token_b_contract_addr
        assert_eq!(pair_info.asset_infos[0], AssetInfo::Token { contract_addr: token_a_contract_addr.clone() });
        assert_eq!(pair_info.asset_infos[1], AssetInfo::Token { contract_addr: token_b_contract_addr.clone() });

        // the liquidity_token of the pair info should be the lp_token_contract_addr
        assert_eq!(pair_info.liquidity_token, lp_token_contract_addr.clone());

        // PROVIDE LIQUIDITY
        // prepare message to approve the pair contract to spend 10000000 cw20 token_a
        let msg = Cw20ExecuteMsg::IncreaseAllowance {
            spender: pair_contract_addr.clone(),
            amount: Uint128::from(10000000u128),
            expires: None,
        };

        // execute the approve message on the token_a contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(token_a_contract_addr.clone()),
            &msg,
            &[]
        ).unwrap();

        // prepare message to approve the pair contract to spend 10000000 cw20 token_b
        let msg = Cw20ExecuteMsg::IncreaseAllowance {
            spender: pair_contract_addr.clone(),
            amount: Uint128::from(10000000u128),
            expires: None,
        };

        // execute the approve message on the token_b contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(token_b_contract_addr.clone()),
            &msg,
            &[]
        ).unwrap();

        // prepare the add liquidity message to add 10000000 token_a and 10000000 token_b
        let msg = PairExecuteMsg::ProvideLiquidity {
            assets: [
                Asset {
                    info: AssetInfo::Token {
                        contract_addr: "wrong_contract_addr".to_string()
                    },
                    amount: Uint128::from(10000000u128),
                },
                Asset {
                    info: AssetInfo::Token {
                        contract_addr: token_b_contract_addr.clone()
                    },
                    amount: Uint128::from(10000000u128),
                },
            ],
            slippage_tolerance: None,
            receiver: None,
        };

        // execute the add liquidity message on the pair contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(pair_contract_addr.clone()),
            &msg,
            &[]
        );
    }

    // test to add liquidity to a pair of cw20 tokens
    #[test]
    fn provide_liquidity_successfully() {
        // instantiate contracts
        let (mut app,
            token_a_contract_addr,
            token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            code_ids
        ) = instantiate_contracts();

        // create message to create new pair
        let msg = FactoryExecuteMsg::CreatePair {
            asset_infos: [
                AssetInfo::Token {
                    contract_addr: token_a_contract_addr.clone()
                },
                AssetInfo::Token {
                    contract_addr: token_b_contract_addr.clone()
                },
            ],
        };

        // execute create pair message on factory contract
        let res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        ).unwrap();

        // define variables to store the contract addresses of the new pair and lp token
        let mut pair_contract_addr: String = "".to_string();
        let mut lp_token_contract_addr: String = "".to_string();

        // parse events from response
        let events = res.events;

        // loop through events and check the attributes of each event
        for event in events {
            // if the ty of the event is "instantiate"
            if event.ty == "instantiate" {
                // declare a variable to store the contract address temporarily
                let mut temp_contract_addr: String = "".to_string();
                // declare a variable to store the code id temporarily
                let mut temp_code_id: String = "".to_string();

                // loop through the attributes of the event
                for attribute in event.attributes {
                    // if the key of the attribute is "_contract_addr" or "code_id"
                    if attribute.key == "_contract_addr" {
                        // set the value of the attribute to the pair_contract_addr variable
                        temp_contract_addr = attribute.value;
                    } else if attribute.key == "code_id" {
                        // set the value of the attribute to the code_id variable
                        temp_code_id = attribute.value;
                    }
                }

                // if the code_id is equal to the code_id of the pair contract
                if temp_code_id == code_ids.halo_pair_code_id.to_string() {
                    // set the value of the pair_contract_addr variable
                    pair_contract_addr = temp_contract_addr;
                } else if temp_code_id == code_ids.halo_token_code_id.to_string() {
                    // set the value of the lp_token_contract_addr variable
                    lp_token_contract_addr = temp_contract_addr;
                }
            }
        }

        // prepare the query pair message
        let msg = PairQueryMsg::Pair {};

        // query the pair info of pair contract
        let pair_info: PairInfo = app.wrap().query_wasm_smart(pair_contract_addr.clone(), &msg).unwrap();

        // the asset_infos of the pair info should include the token_a_contract_addr and token_b_contract_addr
        assert_eq!(pair_info.asset_infos[0], AssetInfo::Token { contract_addr: token_a_contract_addr.clone() });
        assert_eq!(pair_info.asset_infos[1], AssetInfo::Token { contract_addr: token_b_contract_addr.clone() });

        // the liquidity_token of the pair info should be the lp_token_contract_addr
        assert_eq!(pair_info.liquidity_token, lp_token_contract_addr.clone());

        // PROVIDE LIQUIDITY
        // prepare message to approve the pair contract to spend 10000000 cw20 token_a
        let msg = Cw20ExecuteMsg::IncreaseAllowance {
            spender: pair_contract_addr.clone(),
            amount: Uint128::from(10000000u128),
            expires: None,
        };

        // execute the approve message on the token_a contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(token_a_contract_addr.clone()),
            &msg,
            &[]
        ).unwrap();

        // prepare message to approve the pair contract to spend 10000000 cw20 token_b
        let msg = Cw20ExecuteMsg::IncreaseAllowance {
            spender: pair_contract_addr.clone(),
            amount: Uint128::from(10000000u128),
            expires: None,
        };

        // execute the approve message on the token_b contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(token_b_contract_addr.clone()),
            &msg,
            &[]
        ).unwrap();

        // prepare the add liquidity message to add 10000000 token_a and 10000000 token_b
        let msg = PairExecuteMsg::ProvideLiquidity {
            assets: [
                Asset {
                    info: AssetInfo::Token {
                        contract_addr: token_a_contract_addr.clone()
                    },
                    amount: Uint128::from(10000000u128),
                },
                Asset {
                    info: AssetInfo::Token {
                        contract_addr: token_b_contract_addr.clone()
                    },
                    amount: Uint128::from(10000000u128),
                },
            ],
            slippage_tolerance: None,
            receiver: None,
        };

        // execute the add liquidity message on the pair contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(pair_contract_addr.clone()),
            &msg,
            &[]
        ).unwrap();

        // prepare the query the balance of token_a of the pair contract message
        let msg = Cw20QueryMsg::Balance {
            address: pair_contract_addr.clone(),
        };

        // query the balance of token_a of the pair contract
        let balance: BalanceResponse = app.wrap().query_wasm_smart(token_a_contract_addr.clone(), &msg).unwrap();

        // the balance of token_a of the pair contract should be 10000000
        assert_eq!(balance.balance, Uint128::from(10000000u128));

        // prepare the query the balance of token_b of the pair contract message
        let msg = Cw20QueryMsg::Balance {
            address: pair_contract_addr.clone(),
        };

        // query the balance of token_b of the pair contract
        let balance: BalanceResponse = app.wrap().query_wasm_smart(token_b_contract_addr.clone(), &msg).unwrap();

        // the balance of token_b of the pair contract should be 10000000
        assert_eq!(balance.balance, Uint128::from(10000000u128));

        // prepare the query the balance of lp_token of the ADMIN message
        let msg = Cw20QueryMsg::Balance {
            address: ADMIN.to_string(),
        };

        // query the balance of lp_token of the ADMIN
        let balance: BalanceResponse = app.wrap().query_wasm_smart(lp_token_contract_addr.clone(), &msg).unwrap();

        // the balance of lp_token of the ADMIN should be 10000000
        assert_eq!(balance.balance, Uint128::from(10000000u128));
    }    
}