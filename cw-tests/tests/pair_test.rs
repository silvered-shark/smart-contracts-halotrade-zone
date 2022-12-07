use cosmwasm_std::{Addr, Uint128, Coin};
use cw_multi_test::Executor;
use tests::environment::{ADMIN, NATIVE_DENOM, instantiate_contracts};
use haloswap::factory::ExecuteMsg as FactoryExecuteMsg;
use haloswap::asset::AssetInfo;

#[test]
fn test_add_new_pair() {
    // instantiate contracts
    let (mut app,
        token_a_contract_addr,
        token_b_contract_addr,
        swap_factory_contract_addr,
        swap_router_contract_addr,
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
        &[Coin{denom: NATIVE_DENOM.to_string(), amount: Uint128::new(1u128)}].to_vec()
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
            // loop through the attributes of the event
            for attribute in event.attributes {
                
            }
        }
    }

    println!("token_a_contract_addr: {:?}", token_a_contract_addr);
    println!("token_b_contract_addr: {:?}", token_b_contract_addr);
    println!("swap_factory_contract_addr: {:?}", swap_factory_contract_addr);
    println!("swap_router_contract_addr: {:?}", swap_router_contract_addr);
    assert_eq!(true, false);
}

// [
//     Event { ty: "execute", attributes: [Attribute { key: "_contract_addr", value: "contract2" }] }, 
//     Event { ty: "wasm", attributes: [Attribute { key: "_contract_addr", value: "contract2" }, Attribute { key: "action", value: "create_pair" }, Attribute { key: "pair", value: "contract0-contract1" }] }, 
//     Event { ty: "instantiate", attributes: [Attribute { key: "_contract_addr", value: "contract4" }, Attribute { key: "code_id", value: "2" }] }, 
//     Event { ty: "instantiate", attributes: [Attribute { key: "_contract_addr", value: "contract5" }, Attribute { key: "code_id", value: "1" }] }, 
//     Event { ty: "reply", attributes: [Attribute { key: "_contract_addr", value: "contract4" }, Attribute { key: "mode", value: "handle_success" }] }, Event { ty: "wasm", attributes: [Attribute { key: "_contract_addr", value: "contract4" }, Attribute { key: "liquidity_token_addr", value: "contract5" }] }, Event { ty: "reply", attributes: [Attribute { key: "_contract_addr", value: "contract2" }, Attribute { key: "mode", value: "handle_success" }] }, Event { ty: "wasm", attributes: [Attribute { key: "_contract_addr", value: "contract2" }, Attribute { key: "pair_contract_addr", value: "contract4" }, Attribute { key: "liquidity_token_addr", value: "contract5" }] }]


// AppResponse { 
//     events: [
//         Event { 
//             ty: "execute", 
//             attributes: [
//                 Attribute { 
//                     key: "_contract_addr", 
//                     value: "contract2" 
//                 }
//             ] 
//         }, 
//         Event { 
//             ty: "wasm", 
//             attributes: [
//                 Attribute { 
//                     key: "_contract_addr", 
//                     value: "contract2" 
//                 }, 
//                 Attribute { 
//                     key: "action", 
//                     value: "create_pair" 
//                 }, 
//                 Attribute { 
//                     key: "pair", 
//                     value: "contract0-contract1" 
//                 }
//             ] 
//         }, 
//         Event { 
//             ty: "instantiate", 
//             attributes: [
//                 Attribute { 
//                     key: "_contract_addr", 
//                     value: "contract4" 
//                 }, 
//                 Attribute { 
//                     key: "code_id", 
//                     value: "2" 
//                 }
//             ] 
//         }, 
//         Event { 
//             ty: "instantiate", 
//             attributes: [
//                 Attribute { 
//                     key: "_contract_addr", 
//                     value: "contract5" 
//                 }, 
//                 Attribute { 
//                     key: "code_id", 
//                     value: "1" 
//                 }
//             ] 
//         }, 
//         Event { 
//             ty: "reply", 
//             attributes: [
//                 Attribute { 
//                     key: "_contract_addr", 
//                     value: "contract4" 
//                 }, 
//                 Attribute { 
//                     key: "mode", 
//                     value: "handle_success" 
//                 }
//             ] 
//         }, 
//         Event { 
//             ty: "wasm", 
//             attributes: [
//                 Attribute { 
//                     key: "_contract_addr", 
//                     value: "contract4" 
//                 }, 
//                 Attribute { 
//                     key: "liquidity_token_addr", 
//                     value: "contract5" }
//             ] 
//         }, 
//         Event { 
//             ty: "reply", 
//             attributes: [
//                 Attribute { 
//                     key: "_contract_addr", 
//                     value: "contract2" 
//                 }, 
//                 Attribute { 
//                     key: "mode", 
//                     value: "handle_success" 
//                 }
//             ] 
//         }, 
//         Event { 
//             ty: "wasm", 
//             attributes: [
//                 Attribute { 
//                     key: "_contract_addr", 
//                     value: "contract2" 
//                 }, 
//                 Attribute { 
//                     key: "pair_contract_addr", 
//                     value: "contract4" 
//                 }, 
//                 Attribute { 
//                     key: "liquidity_token_addr", 
//                     value: "contract5" 
//                 }
//             ] 
//         }
//     ], 
//     data: None 
// }