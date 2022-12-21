#[cfg(feature = "secret")]
use {
    crate::state_secret::{OPTION_1, OPTION_2, VOTERS},
    secret_std::{
        entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
        StdResult, Uint128,
    },
};

#[cfg(feature = "vanilla")]
use {
    crate::state_vanilla::{OPTION_1, OPTION_2, VOTERS},
    cosmwasm_std::{
        entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
        StdResult, Uint128,
    },
};

use crate::msg::{ExecuteMsg, InstantiateMsg, OptionTally, QueryMsg, QueryRes};

const PAGE_SIZE: usize = 20;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    OPTION_1.save(deps.storage, &0)?;
    OPTION_2.save(deps.storage, &0)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Vote { option } => vote_impl(deps, info, option),
    }
}

#[cfg(feature = "secret")]
fn vote_impl(deps: DepsMut, info: MessageInfo, option: u64) -> StdResult<Response> {
    if VOTERS.contains(deps.storage, &info.sender) {
        return Err(StdError::generic_err("already voted"));
    }

    VOTERS.insert(deps.storage, &info.sender, &1 /* arbitrary value */)?;

    // Update tally
    match option {
        1 => OPTION_1.update(deps.storage, |tally| Ok(tally + 1))?,
        2 => OPTION_2.update(deps.storage, |tally| Ok(tally + 1))?,
        _ => return Err(StdError::generic_err("unsupported option")),
    };

    Ok(Response::default())
}

#[cfg(feature = "vanilla")]
fn vote_impl(deps: DepsMut, info: MessageInfo, option: u64) -> StdResult<Response> {
    if VOTERS.has(deps.storage, info.sender.clone()) {
        return Err(StdError::generic_err("already voted"));
    }

    VOTERS.save(deps.storage, info.sender, &1 /* arbitrary value */)?;

    // Update tally
    match option {
        1 => OPTION_1.update(deps.storage, |tally| Ok::<u128, StdError>(tally + 1))?,
        2 => OPTION_2.update(deps.storage, |tally| Ok::<u128, StdError>(tally + 1))?,
        _ => return Err(StdError::generic_err("unsupported option")),
    };

    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let res = match msg {
        QueryMsg::Tally {} => QueryRes::Tally {
            option_1: OptionTally {
                option: 1,
                tally: Uint128::from(OPTION_1.load(deps.storage)?),
            },
            option_2: OptionTally {
                option: 2,
                tally: Uint128::from(OPTION_2.load(deps.storage)?),
            },
        },
        QueryMsg::Voters { page } => voters_query_impl(deps, page.unwrap_or(0))?,
    };

    Ok(to_binary(&res)?)
}

#[cfg(feature = "secret")]
fn voters_query_impl(deps: Deps, page: u32) -> StdResult<QueryRes> {
    let voters = VOTERS.paging_keys(deps.storage, page, PAGE_SIZE as u32)?;
    Ok(QueryRes::Voters { voters })
}

#[cfg(feature = "vanilla")]
fn voters_query_impl(deps: Deps, page: u32) -> StdResult<QueryRes> {
    use cosmwasm_std::Addr;

    let voters_iter = VOTERS.keys(deps.storage, None, None, cosmwasm_std::Order::Ascending); //.paging_keys(deps.storage, page, 20)?;
    let voters: Vec<Addr> = voters_iter
        .skip((page as usize) * PAGE_SIZE)
        .take(PAGE_SIZE)
        .filter(|v| v.is_ok())
        .map(|v| v.unwrap())
        .collect();
    Ok(QueryRes::Voters { voters: voters })
}

#[entry_point]
pub fn reply(_deps: DepsMut, _env: Env, _reply: Reply) -> StdResult<Response> {
    unimplemented!()
}
