use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128};

use crate::state::BalanceState;

#[cw_serde]
pub struct InstantiateMsg {
    uninmp: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    WithdrawAvailableShare { coin: Coin },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetBalanceStateResponse)]
    GetBalanceState {},

    #[returns(GetReleaseStateResponse)]
    GetReleaseState {},
}

#[cw_serde]
pub struct GetBalanceStateResponse {
    pub balance_state_per_denom: Vec<BalanceStatePerDenom>,
}

#[cw_serde]
pub struct BalanceStatePerDenom {
    pub denom: String,
    pub balance_state: BalanceState,
}

#[cw_serde]
pub struct GetReleaseStateResponse {
    pub release_state_per_denom: Vec<ReleaseStatePerDenom>,
}

#[cw_serde]
pub struct ReleaseStatePerDenom {
    pub denom: String,
    pub released_amount: Uint128,
    pub not_released_amount: Uint128,
}
