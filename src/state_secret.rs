#![cfg(feature = "secret")]

use crate::state::{OPTION_1_PREFIX, VOTE_PREFIX};
use secret_std::Addr;
use secret_toolkit::storage::{Item, Keymap};

pub static VOTERS: Keymap<Addr, u8> = Keymap::new(VOTE_PREFIX.as_bytes());
pub static OPTION_1: Item<u128> = Item::new(OPTION_1_PREFIX.as_bytes());
pub static OPTION_2: Item<u128> = Item::new(OPTION_1_PREFIX.as_bytes());
