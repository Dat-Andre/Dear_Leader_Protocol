use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    uninmp: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UserAccountVote { proposal_id: u64, vote_option: u64 },
    DearLeaderVote { proposal_id: u64, vote_option: u64 },
    TransferVotePower { dear_leader_addr: String },
    ReclaimVotePower {},
    RegisterDearLeader { new_dear_leader_addr: String },
    RegisterUserAccount {},
    UnregisterUserAccount {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
