use cosmwasm_schema::cw_serde;

use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};
use cw_utils::Expiration;

// Tracks the current balance state of the contract per Denom/Cw20Adress in the current contract
pub const BALANCE_STATE: Map<String, BalanceState> = Map::new("balance_state");

pub const RELEASE_SCHEDULE: Map<String, Vec<ReleaseSchedule>> = Map::new("release_schedule");

#[cw_serde]
pub struct BalanceState {
    pub initial_balance: Uint128,
    pub current_balance: Uint128,
}

#[cw_serde]
pub struct ReleaseSchedule {
    pub locked_until: Expiration,
    pub amount_locked: Uint128,
}
