#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// 06 Instantiate
// - // use cw2::set_contract_version;
// + use cw2::set_contract_version;
use cw2::set_contract_version;
// 06 Instantiate
// + use crate::state::{Config, CONFIG};
use crate::state::{Config, CONFIG};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// 06 Instantiate
// - /*
// - const CONTRACT_NAME: &str = "crates.io:cw-starter";
// - const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
//  - */
// + const CONTRACT_NAME: &str = "crates.io:cw-starter";
// + const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// 06 Instantiate
// - _deps: DepsMut,
// - _env: Env,
// - _info: MessageInfo,
// - _msg: InstantiateMsg,
// + deps: DepsMut,
// + _env: Env,
// + info: MessageInfo,
// + msg: InstantiateMsg,
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // 06 Instantiate
    // - unimplemented!()
    // + set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // + let admin = msg.admin.unwrap_or(info.sender.to_string());
    // + let validated_admin = deps.api.addr_validate(&admin)?;
    // + let config = Config {
    // +     admin: validated_admin.clone(),
    // + };
    // + CONFIG.save(deps.storage, &config)?;
    // + Ok(Response::new()
    // +     .add_attribute("action", "instantiate")
    // +     .add_attribute("admin", validated_admin.to_string()))
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg.admin.unwrap_or(info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;
    let config = Config {
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
