use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    uninmp: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    WithdrawAvailableShare {
        coin: Coin,
        asset_manager_addr: String,
    },
    WithdrawAllAvailableShare {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetAllContractsUnderManagementResponse)]
    GetAllContractsUnderManagement {},
}

#[cw_serde]
pub struct GetAllContractsUnderManagementResponse {
    pub contract_addrs: Vec<String>,
}
