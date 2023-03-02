#[cfg(test)]
mod tests {
    use cosmwasm_std::{coins, from_binary, to_binary, Addr, DepsMut, Response, OwnedDeps, MemoryStorage, testing::{MockApi, MockQuerier, mock_dependencies, mock_info, mock_env}, WasmQuery, Uint128, ContractResult};

    use cw20_base::{ContractError, msg::QueryMsg, msg::ExecuteMsg};
    
    use crate::env_setup::env_setup::{instantiate_contracts, ADMIN, USER_1};
    use crate::contract::*;
    use cw20::{Expiration as Cw20Expiration, TokenInfoResponse, Cw20Coin, MinterResponse};
    use haloswap::token::InstantiateMsg;

    const MOCK_HALO_TOKEN_ADDR: &str = "halo_token_addr";

    const MOCK_OFFER_CW20_ADDR: &str = "cw20_addr";
    const MOCK_OFFER_CW20_AMOUNT: u128 = 1000000000;
    const MOCK_OFFER_CW20_AMOUNT_MINIMUM: u128 = 1;
    const MOCK_OFFER_CW20_PRICE: u128 = 10000000;


    const MOCK_OFFER_NFT_OFFERER_INSUFFICIENT_BALANCE: &str = "offerer 2";
    const MOCK_OFFER_NFT_OFFERER_INSUFFICIENT_ALLOWANCE: &str = "offerer 3";

    fn mock_deps() -> OwnedDeps<MemoryStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            name: "Cafe Token".to_string(),
            symbol: "CAFE".to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                minter: ADMIN.to_string(),
                cap: Some(Uint128::new(90)),
            }),
        };

        // mock querier
        deps.querier.update_wasm(|query| {
            match query {
                WasmQuery::Smart { contract_addr, msg } => match contract_addr.as_str() {
                    MOCK_HALO_TOKEN_ADDR => {
                        let query_msg = from_binary::<cw20_base::msg::QueryMsg>(msg).unwrap();
                        match query_msg {
                            cw20_base::msg::QueryMsg::Balance { address, .. } => {
                                if address == MOCK_OFFER_NFT_OFFERER_INSUFFICIENT_BALANCE {
                                    let result = ContractResult::Ok(
                                        to_binary(&cw20::BalanceResponse {
                                            balance: Uint128::from(MOCK_OFFER_CW20_AMOUNT_MINIMUM),
                                        })
                                        .unwrap(),
                                    );
                                    cosmwasm_std::SystemResult::Ok(result)
                                } else {
                                    let result = ContractResult::Ok(
                                        to_binary(&cw20::BalanceResponse {
                                            balance: Uint128::from(MOCK_OFFER_CW20_AMOUNT),
                                        })
                                        .unwrap(),
                                    );
                                    cosmwasm_std::SystemResult::Ok(result)
                                }
                            }
                            cw20_base::msg::QueryMsg::Allowance { owner, spender: _ } => {
                                if owner == MOCK_OFFER_NFT_OFFERER_INSUFFICIENT_ALLOWANCE {
                                    let result = ContractResult::Ok(
                                        to_binary(&cw20::AllowanceResponse {
                                            allowance: Uint128::from(
                                                MOCK_OFFER_CW20_AMOUNT_MINIMUM,
                                            ),
                                            expires: Cw20Expiration::Never {},
                                        })
                                        .unwrap(),
                                    );
                                    cosmwasm_std::SystemResult::Ok(result)
                                } else {
                                    let result = ContractResult::Ok(
                                        to_binary(&cw20::AllowanceResponse {
                                            allowance: Uint128::from(MOCK_OFFER_CW20_AMOUNT),
                                            expires: Cw20Expiration::Never {},
                                        })
                                        .unwrap(),
                                    );
                                    cosmwasm_std::SystemResult::Ok(result)
                                }
                            }
                            _ => {
                                let result = ContractResult::Err("Not Found".to_string());
                                cosmwasm_std::SystemResult::Ok(result)
                            }
                        }
                    }
                    _ => {
                        panic!("Unexpected contract address: {}", contract_addr);
                    }
                },
                _ => panic!("Unexpected query"),
            }
            // mock query royalty info
        });
        let res = instantiate_contract(deps.as_mut(), msg).unwrap();
        assert_eq!(0, res.messages.len());
        deps
    }

    // we will instantiate a contract with account "owner" but admin is "owner"
    fn instantiate_contract(deps: DepsMut, msg: InstantiateMsg) -> Result<Response, ContractError> {
        let info = mock_info("owner", &coins(1000, "uaura"));
        instantiate(deps, mock_env(), info, msg)
    }

    #[test]
    fn proper_initialization() {
        let deps = mock_deps();
        // query config
        let res = query(deps.as_ref(), mock_env(), QueryMsg::TokenInfo {}).unwrap();
        let token_info: TokenInfoResponse = from_binary(&res).unwrap();

        assert_eq!("Cafe Token".to_string(), token_info.name);
        assert_eq!("CAFE".to_string(), token_info.symbol);
        assert_eq!(6, token_info.decimals);
        assert_eq!(Uint128::from(0u128), token_info.total_supply);
    }

    #[test]
    fn proper_initialization_with_no_minter_data() {
        let msg = InstantiateMsg {
            name: "Cafe Token".to_string(),
            symbol: "CAFE".to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: None, // no minter data
        };
        let mut deps = mock_deps();

        let res = instantiate_contract(deps.as_mut(), msg);
        assert!(res.is_ok());
    }


    mod execute_contract {
        use super::*;
        // Checking Minting works
        fn create_mint_msg(
            deps: DepsMut,
            sender: &str,
            recipient: &str,
            amount: Uint128,
        ) -> Result<Response, ContractError> {
            let msg = ExecuteMsg::Mint {
                recipient: recipient.to_string(),
                amount,
            };
            let info = mock_info(sender, &coins(1000, "uaura"));
            execute(deps, mock_env(), info, msg)
        }

        #[test]
        fn proper_execute_mint() {
            let mut deps = mock_deps();
            let response = create_mint_msg(
                deps.as_mut(), 
                ADMIN, 
                USER_1,
                Uint128::from(10u128));
            
            println!("Response: {:?}", &response);
            assert!(response.is_ok());

        }
    }

}