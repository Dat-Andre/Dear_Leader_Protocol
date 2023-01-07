use cw_storage_plus::{Item, Map};

// relation between dear_leader_account and the list of vote delegatores.
pub const DEAR_LEADER_BOARD: Map<String, Option<Vec<String>>> = Map::new("dear_leader_board");

// relation between user_account and if the vote power is delegated, and if so to whom.
pub const BOSS_VOTE_POWER: Map<String, Option<String>> = Map::new("boss_vote_power");

//relation between proposals and user_accounts
pub const PROPOSAL_VOTE_HISTORY: Map<u64, Vec<String>> = Map::new("proposal_vote_history");

// address of the dear_leader_account factory
pub const DEAR_LEADER_ACCOUNT_FACTORY: Item<String> = Item::new("dear_leader_account_factory");
