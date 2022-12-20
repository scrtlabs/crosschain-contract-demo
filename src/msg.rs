use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "secret")]
use secret_std::{Addr, Uint128};

#[cfg(feature = "vanilla")]
use cosmwasm_std::{Addr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Vote { option: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Tally {},
    Voters { page: Option<u32> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryRes {
    Tally {
        option_1: OptionTally,
        option_2: OptionTally,
    },
    Voters {
        voters: Vec<Addr>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OptionTally {
    pub option: u64,
    pub tally: Uint128,
}
