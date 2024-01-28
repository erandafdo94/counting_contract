use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};

mod contract;
pub mod msg;
mod state;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut, //windows to the outer world
    _env: Env,      //current blockchain state
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;

    match msg {
        Value {} => to_json_binary(&contract::query::value()),
    }
}
