use cosmwasm_std::{Addr, Uint128, Coin, StdError};
use cw_multi_test::Executor;
use tests::environment::{ADMIN, NATIVE_DENOM, instantiate_contracts};
use haloswap::factory::{ExecuteMsg as FactoryExecuteMsg, QueryMsg as FactoryQueryMsg};
use haloswap::asset::{AssetInfo, PairInfo};
use haloswap::pair::QueryMsg as PairQueryMsg;

// pub fn create_pair(
//     mut app: App,
//     factory_addr: String,
//     token_a_addr: String,
//     token_b_addr: String,
// ) -> AnyResult<AppResponse> {
//     // create message to create new pair
//     let msg = FactoryExecuteMsg::CreatePair {
//         asset_infos: [
//             AssetInfo::Token { 
//                 contract_addr: token_a_addr.to_string()
//             },
//             AssetInfo::Token { 
//                 contract_addr: token_b_addr.to_string() 
//             },
//         ],
//     };

//     // execute create pair message on factory contract
//     app.execute_contract(
//         Addr::unchecked(ADMIN),
//         Addr::unchecked(factory_addr.clone()),
//         &msg,
//         &[]
//     )
// }

// module to test creating the pair between cw20 and cw20
mod create_pair_cw20_and_cw20 {
    use super::*;
    // asset 1 is invalid
    #[test]
    fn asset_a_is_invalid() {
        // instantiate contracts
        let (mut app,
            _token_a_contract_addr,
            token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            _code_ids
        ) = instantiate_contracts();

        // create message to create new pair
        let msg = FactoryExecuteMsg::CreatePair {
            asset_infos: [
                AssetInfo::Token { 
                    contract_addr: "invalid".to_string()
                },
                AssetInfo::Token { 
                    contract_addr: token_b_contract_addr.to_string() 
                },
            ],
        };

        // execute create pair message on factory contract
        let res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        );

        assert_eq!(res.unwrap_err().source().unwrap().to_string(), StdError::generic_err("asset1 is invalid").to_string());
    }

    // asset 2 is invalid
    #[test]
    fn asset_b_is_invalid() {
        // instantiate contracts
        let (mut app,
            token_a_contract_addr,
            _token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            _code_ids
        ) = instantiate_contracts();

        // create message to create new pair
        let msg = FactoryExecuteMsg::CreatePair {
            asset_infos: [
                AssetInfo::Token { 
                    contract_addr: token_a_contract_addr.to_string()
                },
                AssetInfo::Token { 
                    contract_addr: "invalid".to_string()
                },
            ],
        };

        // execute create pair message on factory contract
        let res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        );

        assert_eq!(res.unwrap_err().source().unwrap().to_string(), StdError::generic_err("asset2 is invalid").to_string());
    }

    // cannot create a new pair with the same token
    #[test]
    fn cannot_create_new_pair_with_same_token() {
        // instantiate contracts
        let (mut app,
            token_a_contract_addr,
            _token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            _code_ids
        ) = instantiate_contracts();

        // create message to create new pair
        let msg = FactoryExecuteMsg::CreatePair {
            asset_infos: [
                AssetInfo::Token {
                    contract_addr: token_a_contract_addr.clone()
                },
                AssetInfo::Token {
                    contract_addr: token_a_contract_addr.clone()
                },
            ],
        };

        // execute create pair message on factory contract
        let res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        );

        assert_eq!(res.unwrap_err().source().unwrap().to_string(), StdError::generic_err("same asset").to_string());
    }

    // test to create a new pair successfully
    #[test]
    fn create_new_pair_successfully() {
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
    }

    // cannot create a new pair if the pair already exists
    #[test]
    fn cannot_create_new_pair_if_pair_already_exists() {
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

        // create message to create new pair again
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
        );

        assert_eq!(res.unwrap_err().source().unwrap().to_string(), StdError::generic_err("Pair already exists").to_string());
    }

    // cannot create an already existed pair by reversing the order of the asset_infos
    #[test]
    fn cannot_create_already_existed_pair_by_reversing_order_of_asset_infos() {
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

        // create message to create new pair again by reversing the order of the asset_infos
        let msg = FactoryExecuteMsg::CreatePair {
            asset_infos: [
                AssetInfo::Token {
                    contract_addr: token_b_contract_addr.clone()
                },
                AssetInfo::Token {
                    contract_addr: token_a_contract_addr.clone()
                },
            ],
        };

        // execute create pair message on factory contract
        let res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        );

        assert_eq!(res.unwrap_err().source().unwrap().to_string(), StdError::generic_err("Pair already exists").to_string());
    }
}

// module to test adding the pair between native and cw20
mod create_pair_native_and_cw20 {
    use haloswap::factory::NativeTokenDecimalsResponse;
    use tests::environment::USER;

    use super::*;

    // cannot create a new pair between native and cw20 if the native token decimals are not set
    #[test]
    fn cannot_create_new_pair_if_native_token_decimals_not_set() {
        // instantiate contracts
        let (mut app,
            token_a_contract_addr,
            _token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            _code_ids
        ) = instantiate_contracts();

        // create message to create new pair
        let msg = FactoryExecuteMsg::CreatePair {
            asset_infos: [
                AssetInfo::NativeToken {
                    denom: NATIVE_DENOM.to_string()
                },
                AssetInfo::Token {
                    contract_addr: token_a_contract_addr.clone()
                },
            ],
        };

        // execute create pair message on factory contract
        let res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        );

        assert_eq!(res.unwrap_err().source().unwrap().to_string(), StdError::generic_err("asset1 is invalid").to_string());
    }

    // cannot set decimals for native token because not owner
    #[test]
    fn cannot_set_decimals_of_native_token_because_not_owner() {
        // instantiate contracts
        let (mut app,
            _token_a_contract_addr,
            _token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            _code_ids
        ) = instantiate_contracts();

        // create message to allow the native token to be used in the pair
        let msg = FactoryExecuteMsg::AddNativeTokenDecimals {
            denom: NATIVE_DENOM.to_string(),
            decimals: 6u8
        };

        // execute add native token decimals message on factory contract
        let res = app.execute_contract(
            Addr::unchecked(USER),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        );

        assert_eq!(res.unwrap_err().source().unwrap().to_string(), StdError::generic_err("unauthorized").to_string());
    }

    // cannot set decimals for native token if the balance of native token in contract is zero
    #[test]
    fn cannot_set_decimals_of_native_token_if_balance_is_zero() {
        // instantiate contracts
        let (mut app,
            _token_a_contract_addr,
            _token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            _code_ids
        ) = instantiate_contracts();

        // create message to allow the native token to be used in the pair
        let msg = FactoryExecuteMsg::AddNativeTokenDecimals {
            denom: NATIVE_DENOM.to_string(),
            decimals: 6u8
        };

        // execute add native token decimals message on factory contract
        let res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        );

        assert_eq!(res.unwrap_err().source().unwrap().to_string(), StdError::generic_err("a balance greater than zero is required by the factory for verification").to_string());
    }

    // can create a new pair between native and cw20
    #[test]
    fn create_new_pair_successfully() {
        // instantiate contracts
        let (mut app,
            token_a_contract_addr,
            _token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            code_ids
        ) = instantiate_contracts();

        // create message to allow the native token to be used in the pair
        let msg = FactoryExecuteMsg::AddNativeTokenDecimals {
            denom: NATIVE_DENOM.to_string(),
            decimals: 6u8
        };

        // execute add native token decimals message on factory contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[Coin{denom: NATIVE_DENOM.to_string(), amount: Uint128::new(1u128)}].to_vec()
        ).unwrap();

        // create message to query the native token decimals
        let msg = FactoryQueryMsg::NativeTokenDecimals {
            denom: NATIVE_DENOM.to_string()
        };

        // query the native token decimals
        let decimals: NativeTokenDecimalsResponse = app.wrap().query_wasm_smart(swap_factory_contract_addr.clone(), &msg).unwrap();

        // the decimals should be 6
        assert_eq!(decimals.decimals, 6u8);

        // create message to create new pair
        let msg = FactoryExecuteMsg::CreatePair {
            asset_infos: [
                AssetInfo::NativeToken {
                    denom: NATIVE_DENOM.to_string()
                },
                AssetInfo::Token {
                    contract_addr: token_a_contract_addr.clone()
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
                    // set the value of the pair
                    pair_contract_addr = temp_contract_addr;
                } else if temp_code_id == code_ids.halo_token_code_id.to_string() {
                    // set the value of the lp token
                    lp_token_contract_addr = temp_contract_addr;
                }
            }
        }

        // prepare the query pair message
        let msg = PairQueryMsg::Pair {};

        // query the pair info of pair contract
        let pair_info: PairInfo = app.wrap().query_wasm_smart(pair_contract_addr.clone(), &msg).unwrap();

        // the asset_infos of the pair info should include the token_a_contract_addr and token_b_contract_addr
        assert_eq!(pair_info.asset_infos[0], AssetInfo::NativeToken { denom: NATIVE_DENOM.to_string() });
        assert_eq!(pair_info.asset_infos[1], AssetInfo::Token { contract_addr: token_a_contract_addr.clone() });

        // the liquidity_token of the pair info should be the lp_token_contract_addr
        assert_eq!(pair_info.liquidity_token, lp_token_contract_addr.clone());

    }
}

// module to test create new pair between native and native
mod create_new_pair_native_and_native {
    use tests::environment::NATIVE_DENOM_2;

    use super::*;

    // can't create a new pair between native and native if the native token decimals are not set
    #[test]
    fn cannot_create_new_pair_if_native_token_decimals_not_set() {
        // instantiate contracts
        let (mut app,
            _token_a_contract_addr,
            _token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            _code_ids
        ) = instantiate_contracts();

        // create message to create new pair
        let msg = FactoryExecuteMsg::CreatePair {
            asset_infos: [
                AssetInfo::NativeToken {
                    denom: NATIVE_DENOM.to_string()
                },
                AssetInfo::NativeToken {
                    denom: NATIVE_DENOM_2.to_string()
                },
            ],
        };

        // execute create pair message on factory contract
        let res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        );

        assert_eq!(res.unwrap_err().source().unwrap().to_string(), StdError::generic_err("asset1 is invalid").to_string());
    }

    // can create a new pair between native and native
    #[test]
    fn create_new_pair_native_native_successfully() {
        // instantiate contracts
        let (mut app,
            _token_a_contract_addr,
            _token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            code_ids
        ) = instantiate_contracts();

        // create message to allow the native token to be used in the pair
        let msg = FactoryExecuteMsg::AddNativeTokenDecimals {
            denom: NATIVE_DENOM.to_string(),
            decimals: 6u8
        };

        // execute add native token decimals message on factory contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[Coin{denom: NATIVE_DENOM.to_string(), amount: Uint128::new(1u128)}].to_vec()
        ).unwrap();

        // create message to allow the native token 2 to be used in the pair
        let msg = FactoryExecuteMsg::AddNativeTokenDecimals {
            denom: NATIVE_DENOM_2.to_string(),
            decimals: 6u8
        };

        // execute add native token 2 decimals message on factory contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[Coin{denom: NATIVE_DENOM_2.to_string(), amount: Uint128::new(1u128)}].to_vec()
        ).unwrap();

        // create message to create new pair
        let msg = FactoryExecuteMsg::CreatePair {
            asset_infos: [
                AssetInfo::NativeToken {
                    denom: NATIVE_DENOM.to_string()
                },
                AssetInfo::NativeToken {
                    denom: NATIVE_DENOM_2.to_string()
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

        // get the events of the response
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
                    // set the value of the pair
                    pair_contract_addr = temp_contract_addr;
                } else if temp_code_id == code_ids.halo_token_code_id.to_string() {
                    // set the value of the lp token
                    lp_token_contract_addr = temp_contract_addr;
                }
            }
        }

        // prepare the query pair message
        let msg = PairQueryMsg::Pair {};

        // query the pair info of pair contract
        let pair_info: PairInfo = app.wrap().query_wasm_smart(pair_contract_addr.clone(), &msg).unwrap();

        // the asset_infos of the pair info should include the token_a_contract_addr and token_b_contract_addr
        assert_eq!(pair_info.asset_infos[0], AssetInfo::NativeToken { denom: NATIVE_DENOM.to_string() });
        assert_eq!(pair_info.asset_infos[1], AssetInfo::NativeToken { denom: NATIVE_DENOM_2.to_string() });

        // the liquidity_token of the pair info should be the lp_token_contract_addr
        assert_eq!(pair_info.liquidity_token, lp_token_contract_addr.clone());

    }
}

mod query_test {
    use haloswap::factory::PairsResponse;
    use tests::environment::NATIVE_DENOM_2;

    use super::*;
    // create 2 pair and query the pairs
    #[test]
    fn query_pairs_successfully() {
        // instantiate contracts
        let (mut app,
            token_a_contract_addr,
            token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            _code_ids
        ) = instantiate_contracts();

        // create message to allow the native token to be used in the pair
        let msg = FactoryExecuteMsg::AddNativeTokenDecimals {
            denom: NATIVE_DENOM.to_string(),
            decimals: 6u8
        };

        // execute add native token decimals message on factory contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[Coin{denom: NATIVE_DENOM.to_string(), amount: Uint128::new(1u128)}].to_vec()
        ).unwrap();

        // create message to allow the native token 2 to be used in the pair
        let msg = FactoryExecuteMsg::AddNativeTokenDecimals {
            denom: NATIVE_DENOM_2.to_string(),
            decimals: 6u8
        };

        // execute add native token 2 decimals message on factory contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[Coin{denom: NATIVE_DENOM_2.to_string(), amount: Uint128::new(1u128)}].to_vec()
        ).unwrap(); 

        // create message to create new pair
        let msg = FactoryExecuteMsg::CreatePair {
            asset_infos: [
                AssetInfo::NativeToken {
                    denom: NATIVE_DENOM.to_string()
                },
                AssetInfo::NativeToken {
                    denom: NATIVE_DENOM_2.to_string()
                },
            ],
        };

        // execute create pair message on factory contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        );

        // create message to create new pair from 2 cw20 tokens
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
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        ).unwrap();

        // prepare the query pairs message
        let msg = FactoryQueryMsg::Pairs {
            start_after: Some([
                AssetInfo::Token {
                    contract_addr: token_a_contract_addr.clone()
                },
                AssetInfo::Token {
                    contract_addr: token_b_contract_addr.clone()
                },
            ]),
            limit: None,
        };

        // query the pairs of the factory contract
        let pairs: PairsResponse = app.wrap().query_wasm_smart(swap_factory_contract_addr.clone(), &msg).unwrap();

        // the pairs should have 2 pairs
        assert_eq!(pairs.pairs.len(), 1);
    }

    // create 2 pair and query the pairs with pagination
    #[test]
    fn query_pairs_with_pagination_successfully() {
        // instantiate contracts
        let (mut app,
            token_a_contract_addr,
            token_b_contract_addr,
            swap_factory_contract_addr,
            _swap_router_contract_addr,
            _code_ids
        ) = instantiate_contracts();

        // create message to allow the native token to be used in the pair
        let msg = FactoryExecuteMsg::AddNativeTokenDecimals {
            denom: NATIVE_DENOM.to_string(),
            decimals: 6u8
        };

        // execute add native token decimals message on factory contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[Coin{denom: NATIVE_DENOM.to_string(), amount: Uint128::new(1u128)}].to_vec()
        ).unwrap();

        // create message to allow the native token 2 to be used in the pair
        let msg = FactoryExecuteMsg::AddNativeTokenDecimals {
            denom: NATIVE_DENOM_2.to_string(),
            decimals: 6u8
        };

        // execute add native token 2 decimals message on factory contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[Coin{denom: NATIVE_DENOM_2.to_string(), amount: Uint128::new(1u128)}].to_vec()
        ).unwrap(); 

        // create message to create new pair
        let msg = FactoryExecuteMsg::CreatePair {
            asset_infos: [
                AssetInfo::NativeToken {
                    denom: NATIVE_DENOM.to_string()
                },
                AssetInfo::NativeToken {
                    denom: NATIVE_DENOM_2.to_string()
                },
            ],
        };

        // execute create pair message on factory contract
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        );

        // create message to create new pair from 2 cw20 tokens
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
        let _res = app.execute_contract(
            Addr::unchecked(ADMIN),
            Addr::unchecked(swap_factory_contract_addr.clone()),
            &msg,
            &[]
        ).unwrap();

        // prepare the query pairs message
        let msg = FactoryQueryMsg::Pairs {
            start_after: None,
            limit: None,
        };

        // query the pairs of the factory contract
        let pairs: PairsResponse = app.wrap().query_wasm_smart(swap_factory_contract_addr.clone(), &msg).unwrap();

        // the pairs should have 2 pairs
        assert_eq!(pairs.pairs.len(), 2);
    }
}