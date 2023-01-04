#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
// const CONTRACT_NAME: &str = "crates.io:user_manager";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
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
        ExecuteMsg::WithdrawAvailableShare {
            coin,
            asset_manager_addr,
        } => execute::withdraw_available_share(deps, info, coin, asset_manager_addr),
        ExecuteMsg::WithdrawAllAvailableShare {} => {
            execute::withdraw_all_available_share(deps, info)
        }
    }
}

pub mod execute {
    use cosmwasm_std::Coin;

    use super::*;

    pub fn withdraw_all_available_share(
        _deps: DepsMut,
        _info: MessageInfo,
    ) -> Result<Response, ContractError> {
        unimplemented!()
    }

    pub fn withdraw_available_share(
        _deps: DepsMut,
        _info: MessageInfo,
        _coin: Coin,
        _asset_manager_addr: String,
    ) -> Result<Response, ContractError> {
        unimplemented!()
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAllContractsUnderManagement {} => {
            to_binary(&query::get_all_contracts_under_management(deps)?)
        }
    }
}

pub mod query {

    use crate::msg::GetAllContractsUnderManagementResponse;

    use super::*;

    pub fn get_all_contracts_under_management(
        _deps: Deps,
    ) -> StdResult<GetAllContractsUnderManagementResponse> {
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
