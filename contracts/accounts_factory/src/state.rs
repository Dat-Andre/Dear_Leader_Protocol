use cw_storage_plus::{Item, Map};

pub const USER_ACCOUNTS_UNDER_MANAGEMENT: Item<Vec<String>> =
    Item::new("user_accounts_under_management");

pub const DEAR_LEADER_ACCOUNTS_UNDER_MANAGEMENT: Item<Vec<String>> =
    Item::new("dear_leader_accounts_under_management");

pub const USER_ACCOUNTS_CODE_ID: Item<u64> = Item::new("user_accounts_code_id");

pub const DEAR_LEADER_ACCOUNTS_CODE_ID: Item<u64> = Item::new("dear_leader_accounts_code_id");

pub const ASSEMBLY_ADDR: Item<String> = Item::new("assembly_code_id");
