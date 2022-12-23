#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:caveat_protocol";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::WithdrawAvailableShare { coin } => {
            execute::withdraw_available_share(deps, info, coin)
        }
    }
}

pub mod execute {
    use cosmwasm_std::Coin;

    use super::*;

    pub fn withdraw_available_share(
        deps: DepsMut,
        info: MessageInfo,
        coin: Coin,
    ) -> Result<Response, ContractError> {
        unimplemented!()
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalanceState {} => to_binary(&query::get_balance_state(deps)?),
        QueryMsg::GetReleaseState {} => to_binary(&query::get_release_state(deps)?),
    }
}

pub mod query {
    use crate::msg::{GetBalanceStateResponse, GetReleaseStateResponse};

    use super::*;

    pub fn get_balance_state(deps: Deps) -> StdResult<GetBalanceStateResponse> {
        unimplemented!()
    }

    pub fn get_release_state(deps: Deps) -> StdResult<GetReleaseStateResponse> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {
        unimplemented!()
    }

    #[test]
    fn test_2() {
        unimplemented!()
    }

    #[test]
    fn test_3() {
        unimplemented!()
    }
}
