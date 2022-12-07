use cw20::BalanceResponse;
use cw20_base::msg::ExecuteMsg as HaloTokenExecuteMsg;
use cosmwasm_std::{Addr, Uint128, Coin};
use cw_multi_test::Executor;
use tests::environment::{ADMIN, USER, NATIVE_DENOM, TOKEN_INITIAL_BALANCE, instantiate_contracts};

#[test]
fn test_transfer() {
    // instantiate contracts
    let (mut app,
        token_a_contract_addr,
        _token_b_contract_addr,
        _swap_factory_contract_addr,
        _swap_router_contract_addr,
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
