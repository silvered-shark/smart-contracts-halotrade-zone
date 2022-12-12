use cw20::{BalanceResponse, Cw20Coin, MinterResponse};
use cw20_base::msg::ExecuteMsg as HaloTokenExecuteMsg;
use cosmwasm_std::{Addr, Uint128, Coin, StdError};
use cw_multi_test::Executor;
use tests::environment::{ADMIN, USER, NATIVE_DENOM, TOKEN_INITIAL_BALANCE, instantiate_contracts};
use haloswap::token::InstantiateMsg as HaloTokenInstantiateMsg;

// cannot instantiate contract with Initial supply greater than cap
#[test]
fn test_instantiate_with_too_large_initial_supply() {
    // instantiate contracts
    let (mut app,
        _token_a_contract_addr,
        _token_b_contract_addr,
        _swap_factory_contract_addr,
        _swap_router_contract_addr,
        code_ids
    ) = instantiate_contracts();

    // create instantiate message for token_A
    let token_a_instantiate_msg = HaloTokenInstantiateMsg {
        name: "Token A".to_string(),
        symbol: "TKA".to_string(),
        decimals: 6,
        initial_balances: [
            Cw20Coin {
                address: ADMIN.to_string(),
                amount: Uint128::new(TOKEN_INITIAL_BALANCE+1),
            },
        ].to_vec(),
        mint: Some(MinterResponse {
            minter: ADMIN.to_string(),
            cap: Some(Uint128::new(TOKEN_INITIAL_BALANCE)),
        }),
    };

    // instantiate token_A
    let res = app
        .instantiate_contract(
            code_ids.halo_token_code_id.clone(),
            Addr::unchecked(ADMIN),
            &token_a_instantiate_msg,
            &[],
            "test instantiate token A",
            None,
        );
    
    assert_eq!(res.unwrap_err().source().unwrap().to_string(), StdError::generic_err("Initial supply greater than cap").to_string());

}

#[test]
fn test_transfer() {
    // instantiate contracts
    let (mut app,
        token_a_contract_addr,
        _token_b_contract_addr,
        _swap_factory_contract_addr,
        _swap_router_contract_addr,
        _code_ids
    ) = instantiate_contracts();

    // create message to query balance of admin
    let request_balance_admin = cw20_base::msg::QueryMsg::Balance {
        address: ADMIN.to_string(),
    };
    // query balance of admin before transfer
    let admin_balance_before: BalanceResponse = app.wrap().query_wasm_smart(token_a_contract_addr.clone(), &request_balance_admin.clone()).unwrap();
    assert_eq!(admin_balance_before.balance, Uint128::new(TOKEN_INITIAL_BALANCE.into()));

    // create transfer message
    let msg = HaloTokenExecuteMsg::Transfer {
        recipient: USER.to_string(),
        amount: Uint128::new(500000000u128.into())
    };

    // execute transfer message
    app.execute_contract(
        Addr::unchecked(ADMIN),
        Addr::unchecked(token_a_contract_addr.clone()),
        &msg,
        &[Coin{denom: NATIVE_DENOM.to_string(), amount: Uint128::new(1u128)}].to_vec()
    ).unwrap();

    // query balance of admin after transfer
    let admin_balance_after: BalanceResponse = app.wrap().query_wasm_smart(token_a_contract_addr.clone(), &request_balance_admin).unwrap();
    assert_eq!(admin_balance_after.balance, Uint128::new((TOKEN_INITIAL_BALANCE - 500000000u128).into()));

    // create message to query balance of admin
    let request_balance_user = cw20_base::msg::QueryMsg::Balance {
        address: USER.to_string(),
    };
    // query balance of user after transfer
    let user_balance: BalanceResponse = app.wrap().query_wasm_smart(token_a_contract_addr.clone(), &request_balance_user).unwrap();
    assert_eq!(user_balance.balance, Uint128::new(500000000u128.into()));
}
