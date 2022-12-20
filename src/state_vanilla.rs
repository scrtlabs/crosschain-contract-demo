#![cfg(feature = "vanilla")]

use crate::state::{OPTION_1_PREFIX, VOTE_PREFIX};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub static VOTERS: Map<Addr, u8> = Map::new(VOTE_PREFIX);
pub static OPTION_1: Item<u128> = Item::new(OPTION_1_PREFIX);
pub static OPTION_2: Item<u128> = Item::new(OPTION_1_PREFIX);
