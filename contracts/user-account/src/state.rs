use cw_storage_plus::Item;

pub const ASSEMBLY_ADDR: Item<String> = Item::new("assembly");
pub const DEAR_LEADER_ADDR: Item<String> = Item::new("dear_leader");
pub const BOSS_ADDR: Item<String> = Item::new("boss");
