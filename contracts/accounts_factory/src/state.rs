use cw_storage_plus::{Item, Map};

pub const ACCOUNTS_UNDER_MANAGEMENT: Item<Vec<String>> = Item::new("accounts_under_management");
