#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use util_types::ContractError as CommonContractError;
use util_types::ExecuteMsg as CommonExecuteMsg;
use util_types::InstantiateMsg as CommonInstantiateMsg;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:accounts_factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const USER_ACCOUNT_REPLY_ID: u64 = 0;
const DEAR_LEADER_ACCOUNT_REPLY_ID: u64 = 1;

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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateUserAccount {} => execute::create_user_account(deps, env, info),
        ExecuteMsg::CreateDearLeaderAccount {} => {
            execute::create_dear_leader_account(deps, env, info)
        } /*  ExecuteMsg::WithdrawAllAvailableShare {} => {
              execute::withdraw_all_available_share(deps, info)
          } */
    }
}

pub mod execute {
    use cosmwasm_std::{Coin, ReplyOn, SubMsg, WasmMsg};

    use crate::state::{
        ASSEMBLY_ADDR, DEAR_LEADER_ACCOUNTS_UNDER_MANAGEMENT, USER_ACCOUNTS_CODE_ID,
        USER_ACCOUNTS_UNDER_MANAGEMENT,
    };

    use super::*;

    pub fn create_dear_leader_account(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        // this accounts can only be created by the admin of the contract

        // validate that the dear_leader don't have an account yet
        if DEAR_LEADER_ACCOUNTS_UNDER_MANAGEMENT
            .load(deps.storage)?
            .contains(&info.sender.to_string())
        {
            return Err(ContractError::DearLeaderAccountAlreadyExists);
        }

        // create msg to register dear leader in the assembly contract
        /*  let assembly_addr = ASSEMBLY_ADDR.load(deps.storage)?;
        let msg = WasmMsg::Execute {
            contract_addr: assembly_addr,
            msg: to_binary(&CommonExecuteMsg::RegisterWannaBe {
                wanna_be_addr: info.sender.to_string(),
            })?,
            funds: vec![],
        };

        let subMsg: SubMsg = SubMsg::reply_on_success(msg, id); */
        Ok(Response::new()
            .add_attribute("action", "create_user_account")
            .add_attribute("created_by", info.sender.to_string())
            .add_message(msg))
    }

    pub fn create_user_account(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        // validate that user don't have an account yet
        if USER_ACCOUNTS_UNDER_MANAGEMENT
            .load(deps.storage)?
            .contains(&info.sender.to_string())
        {
            return Err(ContractError::UserAccountAlreadyExists);
        }

        // create user account with instantiate msg with ReplyOn::Success
        let user_account_code_id = USER_ACCOUNTS_CODE_ID.load(deps.storage)?;
        let instantiate_msg = WasmMsg::Instantiate {
            admin: Some(env.contract.address.to_string()),
            code_id: user_account_code_id,
            msg: to_binary(&CommonInstantiateMsg::InstantiateUserAccountMsg {})?,
            funds: vec![],
            label: info.sender.to_string() + "_user_account",
        };
        let submessage: SubMsg = SubMsg::reply_on_success(instantiate_msg, USER_ACCOUNT_REPLY_ID);

        Ok(Response::new()
            .add_attribute("action", "create_user_account")
            .add_attribute("created_by", info.sender.to_string())
            .add_submessage(submessage))
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        USER_ACCOUNT_REPLY_ID => reply::instantiate_user_account_reply(deps, msg),
        DEAR_LEADER_ACCOUNT_REPLY_ID => reply::instantiate_dear_leader_account_reply(deps, msg),
        _ => Err(ContractError::UnknownReplyIdCommon {}),
    }
}

pub mod reply {
    use crate::state::{
        ASSEMBLY_ADDR, DEAR_LEADER_ACCOUNTS_UNDER_MANAGEMENT, USER_ACCOUNTS_UNDER_MANAGEMENT,
    };

    use super::*;
    use cosmwasm_std::WasmMsg;
    use cw_utils::parse_reply_instantiate_data;
    //use util_types::ContractError as CommonContractError;

    pub fn instantiate_dear_leader_account_reply(
        deps: DepsMut,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let res = parse_reply_instantiate_data(msg)?;
        let dear_leader_account_addr = deps.api.addr_validate(&res.contract_address)?;

        // add dear leader account to the list of dear leader accounts under management
        DEAR_LEADER_ACCOUNTS_UNDER_MANAGEMENT.update(
            deps.storage,
            |mut dear_leader_accounts| -> StdResult<_> {
                dear_leader_accounts.push(dear_leader_account_addr.to_string());
                Ok(dear_leader_accounts)
            },
        )?;

        let assembly_addr = ASSEMBLY_ADDR.load(deps.storage)?;

        let msg = WasmMsg::Execute {
            contract_addr: assembly_addr,
            msg: to_binary(&CommonExecuteMsg::RegisterWannaBe {
                wanna_be_addr: dear_leader_account_addr.to_string(),
            })?,
            funds: vec![],
        };

        Ok(Response::new()
            .add_attribute("action", "instantiated_dear_leader_account_reply")
            .add_attribute(
                "dear_leader_account_ addr",
                dear_leader_account_addr.to_string(),
            )
            .add_message(msg))
    }

    pub fn instantiate_user_account_reply(
        deps: DepsMut,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let res = parse_reply_instantiate_data(msg)?;
        let user_account_addr = deps.api.addr_validate(&res.contract_address)?;

        // add user account to the list of user accounts under management
        USER_ACCOUNTS_UNDER_MANAGEMENT.update(
            deps.storage,
            |mut user_accounts| -> StdResult<_> {
                user_accounts.push(user_account_addr.to_string());
                Ok(user_accounts)
            },
        )?;

        Ok(Response::default()
            .add_attribute("instantiated_user_account", user_account_addr.to_string()))
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
