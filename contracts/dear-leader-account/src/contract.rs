#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, ASSEMBLY_ADDR, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:dear_leader_acount";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: info.sender.to_string(),
    };
    CONFIG.save(deps.storage, &config)?;

    ASSEMBLY_ADDR.save(deps.storage, &msg.assembly_addr)?;
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Vote {
            proposal_id,
            vote_option,
        } => execute::vote(deps, env, info, proposal_id, vote_option),
    }
}

pub mod execute {
    use cosmwasm_std::WasmMsg;

    use crate::state::{ASSEMBLY_ADDR, CONFIG};
    use util_types::ExecuteMsg as ExecuteMsgCommon;

    use super::*;

    pub fn vote(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        proposal_id: u64,
        vote_option: u64,
    ) -> Result<Response, ContractError> {
        // validate contract caller is owner
        validate_owner(deps.as_ref(), &info)?;
        // create vote message to assembly contract
        let msg = WasmMsg::Execute {
            contract_addr: ASSEMBLY_ADDR.load(deps.storage)?,
            msg: to_binary(&ExecuteMsgCommon::DearLeaderVote {
                proposal_id,
                vote_option,
            })?,
            funds: vec![],
        };

        Ok(Response::new()
            .add_attribute("action", "vote")
            .add_message(msg))
    }

    pub fn validate_owner(deps: Deps, info: &MessageInfo) -> Result<(), ContractError> {
        let config = CONFIG.load(deps.storage)?;
        if info.sender != config.owner {
            return Err(ContractError::Unauthorized {});
        }
        Ok(())
    }

    /* pub fn validate_factory(deps: Deps, info: &MessageInfo) -> Result<(), ContractError> {
        let factory = ACCOUNTS_FACTORY_ADDR.load(deps.storage)?;
        if info.sender != factory {
            return Err(ContractError::Unauthorized {});
        }
        Ok(())
    } */
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
