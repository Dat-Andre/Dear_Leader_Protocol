#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg,
};
use cw2::set_contract_version;
use util_types::ExecuteMsg as CommonExecuteMsg;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{DEAR_LEADER_BOARD, PROPOSAL_VOTE_HISTORY};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:assembly";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
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
        ExecuteMsg::UserAccountVote {
            proposal_id,
            vote_option,
        } => execute::user_account_vote(deps, info, proposal_id, vote_option),
        ExecuteMsg::DearLeaderVote {
            proposal_id,
            vote_option,
        } => execute::dear_leader_vote(deps, info, proposal_id, vote_option),
        ExecuteMsg::TransferVotePower { dear_leader_addr } => {
            execute::transfer_vote_power(deps, env, info, dear_leader_addr)
        }
        ExecuteMsg::ReclaimVotePower {} => execute::reclaim_vote_power(deps, env, info),
        ExecuteMsg::RegisterDearLeader {
            new_dear_leader_addr,
        } => execute::register_new_dear_leader(deps, info, new_dear_leader_addr),
        ExecuteMsg::RegisterUserAccount {} => execute::register_user_account(deps, info),
        ExecuteMsg::UnregisterUserAccount {} => execute::unregister_user_account(deps, info),
    }
}

pub mod execute {

    use cosmwasm_std::WasmMsg;

    use crate::state::{
        BOSS_VOTE_POWER, DEAR_LEADER_ACCOUNT_FACTORY, DEAR_LEADER_BOARD, PROPOSAL_VOTE_HISTORY,
    };

    use super::*;

    pub fn unregister_user_account(
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        // check if user accout is registered, and if so, unregister it
        BOSS_VOTE_POWER.update(deps.storage, info.sender.to_string(), |vote_power| {
            if let Some(_) = vote_power {
                // unregister account
                Ok(None)
            } else {
                return Err(ContractError::AccountNotRegistered {});
            }
        })?;

        Ok(Response::default()
            .add_attribute("action", "unregister_user_account")
            .add_attribute("user_account", info.sender.to_string()))
    }

    pub fn register_user_account(
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        // make sure the user account is NOT already registered
        // register new user account
        BOSS_VOTE_POWER.update(deps.storage, info.sender.to_string(), |vote_power| {
            if let None = vote_power {
                // register account
                Ok(None)
            } else {
                return Err(ContractError::UserAccountAlreadyRegister {});
            }
        })?;

        Ok(Response::default()
            .add_attribute("action", "register_user_account")
            .add_attribute("user_account", info.sender.to_string()))
    }

    // When Instantiating the new dear_leader_account contract, it should generate a message to register itself in this contract as a dear_leader
    pub fn register_new_dear_leader(
        deps: DepsMut,
        info: MessageInfo,
        new_dear_leader_addr: String,
    ) -> Result<Response, ContractError> {
        //check if the sender is the dear_leader_account_factory
        let dear_leader_account_factory = DEAR_LEADER_ACCOUNT_FACTORY.load(deps.storage)?;

        if dear_leader_account_factory != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        // check if dear leader is already registered, if not, register him, if yes return error
        DEAR_LEADER_BOARD.update(deps.storage, new_dear_leader_addr.to_string(), |addrs| {
            if let None = addrs {
                // register account
                Ok(None)
            } else {
                return Err(ContractError::AlreadyIsADearLeader {});
            }
        })?;

        Ok(Response::new()
            .add_attribute("action", "register_dear_leader_addr")
            .add_attribute("dear_leader_account_addr", new_dear_leader_addr))
    }

    pub fn transfer_vote_power(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        dear_leader_addr: String,
    ) -> Result<Response, ContractError> {
        // check if dear_leader_addr is valid and is registered in the assembly
        let dear_leader_addr = deps
            .api
            .addr_validate(&dear_leader_addr)
            .map_err(|_| ContractError::InvalidDearLeader {})?;

        /*  let dear_leader_delegators = DEAR_LEADER_BOARD
        .load(deps.storage, dear_leader_addr.to_string())
        .map_err(|_| ContractError::DearLeaderNotRegistered {})?; */
        if !DEAR_LEADER_BOARD.has(deps.storage, dear_leader_addr.to_string()) {
            return Err(ContractError::DearLeaderNotRegistered {});
        }

        // check if the user_account is registered in the assembly
        let current_dear_leader = BOSS_VOTE_POWER
            .load(deps.storage, info.sender.to_string())
            .map_err(|_| ContractError::AccountNotRegistered {})?;

        // check if user_account has a current dear_leader

        match current_dear_leader {
            Some(current_dear_leader) => {
                // remove the user_account from the current dear_leader list of supporters
                // and update the new dear_lear list of supporters
                DEAR_LEADER_BOARD.update(
                    deps.storage,
                    current_dear_leader.to_string(),
                    |addrs_opt| {
                        if let Some(addrs) = addrs_opt {
                            if let Some(mut list) = addrs {
                                list.retain(|addr| addr != &info.sender.to_string());
                                if list.is_empty() {
                                    return Ok(None);
                                }
                                Ok(Some(list))
                            } else {
                                Err(ContractError::InternalErrorInLogic {})
                            }
                        } else {
                            return Err(ContractError::InternalErrorInLogic {});
                        }
                    },
                )?;
            }
            None => {
                // this means the user_account is not currently delegating his vote powers
                BOSS_VOTE_POWER.save(
                    deps.storage,
                    info.sender.to_string(),
                    &Some(dear_leader_addr.to_string()),
                )?;
            }
        }

        Ok(Response::new()
            .add_attribute("action", "transfer_vote_power")
            .add_attribute("dear_leader_addr", dear_leader_addr))
    }

    pub fn reclaim_vote_power(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        // validate that the user_account is registered in the assembly
        /* let value_to_update = BOSS_VOTE_POWER
        .load(deps.storage, info.sender.to_string())
        .map_err(|_| ContractError::AccountNotRegistered {})?; */

        if !BOSS_VOTE_POWER.has(deps.storage, info.sender.to_string()) {
            return Err(ContractError::AccountNotRegistered {});
        }

        // reset the option to None, so that the user_account don't delegate his vote power
        BOSS_VOTE_POWER.save(deps.storage, info.sender.to_string(), &None)?;

        Ok(Response::new().add_attribute("action", "reclaim_vote_power"))
    }

    pub fn user_account_vote(
        deps: DepsMut,
        info: MessageInfo,
        proposal_id: u64,
        vote: u64,
    ) -> Result<Response, ContractError> {
        // check if vote is valid
        if vote < 1 || vote > 4 {
            return Err(ContractError::InvalidVote {});
        }

        //validate that user_account is registered in the assembly
        /* let valid_user_account = BOSS_VOTE_POWER
        .load(deps.storage, info.sender.to_string())
        .map_err(|_| ContractError::AccountNotRegistered {})?; */
        if !BOSS_VOTE_POWER.has(deps.storage, info.sender.to_string()) {
            return Err(ContractError::AccountNotRegistered {});
        }

        // update PROPOSAL_VOTE_HISTORY
        PROPOSAL_VOTE_HISTORY.update(
            deps.storage,
            proposal_id,
            |votes| -> StdResult<Vec<String>> {
                if let Some(mut votes) = votes {
                    votes.push(info.sender.to_string());
                    Ok(votes)
                } else {
                    Ok(vec![info.sender.to_string()])
                }
            },
        )?;

        let msg = WasmMsg::Execute {
            contract_addr: info.sender.to_string(),
            msg: to_binary(&CommonExecuteMsg::AssemblyVote {
                proposal_id,
                vote_option: vote,
            })?,
            funds: vec![],
        };

        Ok(Response::new()
            .add_attribute("action", "vote")
            .add_attribute("proposal_id", proposal_id.to_string())
            .add_attribute("vote", vote.to_string())
            .add_message(msg))
    }

    pub fn dear_leader_vote(
        deps: DepsMut,
        info: MessageInfo,
        proposal_id: u64,
        vote: u64,
    ) -> Result<Response, ContractError> {
        // check if vote is valid
        if vote < 1 || vote > 4 {
            return Err(ContractError::InvalidVote {});
        }

        //validate that dear_leader_account is registered in the assembly and has at least one delegator
        let valid_dear_leader_list_of_delegators = DEAR_LEADER_BOARD
            .load(deps.storage, info.sender.to_string())
            .map_err(|_| ContractError::DearLeaderNotRegistered {})?;

        if valid_dear_leader_list_of_delegators.is_none() {
            return Err(ContractError::NoVotePower {});
        }

        // validate that at least one delegator exists
        // if not, return error
        // if yes, iterate over the list of delegators (also validating he has not voted yet) and create messages
        // prepare list to allow validate that user_account have not voted yet
        let delegators_that_already_voted = PROPOSAL_VOTE_HISTORY
            .may_load(deps.storage, proposal_id)?
            .ok_or(ContractError::ProposalNotRegistered {})?;

        // Note: It is not possible to have a list of 0 elements
        let msgs = valid_dear_leader_list_of_delegators
            .unwrap()
            .iter()
            .filter(|addr| !delegators_that_already_voted.contains(addr))
            .map(|addr| WasmMsg::Execute {
                contract_addr: addr.to_string(),
                msg: to_binary(&CommonExecuteMsg::AssemblyVote {
                    proposal_id,
                    vote_option: vote,
                })
                .unwrap(),
                funds: vec![],
            })
            .take(10) // TEST - HOW MANY MESSAGES CAN BE SENT IN ONE TRANSACTION?
            .collect::<Vec<WasmMsg>>();

        Ok(Response::new()
            .add_attribute("action", "vote")
            .add_attribute("n_of_votes", msgs.len().to_string())
            .add_messages(msgs))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!("Implement query")
}

pub mod query {

    use super::*;
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
