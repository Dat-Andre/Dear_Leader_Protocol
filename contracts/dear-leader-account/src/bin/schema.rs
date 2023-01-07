use cosmwasm_schema::write_api;
use user_account::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// use multi_contract_boilerplate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}