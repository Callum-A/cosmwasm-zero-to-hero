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
mod tests {
    // 07 Instantiate Test
    // + use cosmwasm_std::attr; // helper to construct an attribute e.g. ("action", "instantiate")
    // + use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info}; // mock functions to mock an environment, message info, dependencies
    // + use crate::contract::instantiate; // the contract instantiate function
    // + use crate::msg::InstantiateMsg; // our instantate method

    // + // Two fake addresses we will use to mock_info
    // + pub const ADDR1: &str = "addr1";
    // + pub const ADDR2: &str = "addr2";
    use crate::contract::instantiate; // the contract instantiate function
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::attr; // helper to construct an attribute e.g. ("action", "instantiate")
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info}; // mock functions to mock an environment, message info, dependencies // our instantate method

    // Two fake addresses we will use to mock_info
    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";

    // 07 Instantiate Test
    // + #[test]
    // + fn test_instantiate() {
    // +     // Mock the dependencies, must be mutable so we can pass it as a mutable, empty vector means our contract has no balance
    // +     let mut deps = mock_dependencies();
    // +     // Mock the contract environment, contains the block info, contract address, etc.
    // +     let env = mock_env();
    // +     // Mock the message info, ADDR1 will be the sender, the empty vec means we sent no funds.
    // +     let info = mock_info(ADDR1, &vec![]);
    // +
    // +     // Create a message where we (the sender) will be an admin
    // +     let msg = InstantiateMsg { admin: None };
    // +     // Call instantiate, unwrap to assert success
    // +     let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    // +
    // +     assert_eq!(
    // +         res.attributes,
    // +         vec![attr("action", "instantiate"), attr("admin", ADDR1)]
    // +     )
    // + }
    #[test]
    fn test_instantiate() {
        // Mock the dependencies, must be mutable so we can pass it as a mutable, empty vector means our contract has no balance
        let mut deps = mock_dependencies();
        // Mock the contract environment, contains the block info, contract address, etc.
        let env = mock_env();
        // Mock the message info, ADDR1 will be the sender, the empty vec means we sent no funds.
        let info = mock_info(ADDR1, &vec![]);

        // Create a message where we (the sender) will be an admin
        let msg = InstantiateMsg { admin: None };
        // Call instantiate, unwrap to assert success
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR1)]
        )
    }

    // 07 Instantiate Test Exercise Solution
    // + #[test]
    // + fn test_instantiate_with_admin() {
    // +     let mut deps = mock_dependencies();
    // +     let env = mock_env();
    // +     // Send as ADDR1 to show admin is different
    // +     let info = mock_info(ADDR1, &vec![]);
    // +
    // +     // Create a message where ADDR2 will be an admin
    // +     // Have to use .to_string() method
    // +     let msg = InstantiateMsg {
    // +         admin: Some(ADDR2.to_string()),
    // +     };
    // +     // Unwrap to assert success
    // +     let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    // +     // Assert admin is ADDR2 instead
    // +     assert_eq!(
    // +         res.attributes,
    // +         vec![attr("action", "instantiate"), attr("admin", ADDR2),]
    // +     );
    // + }
    #[test]
    fn test_instantiate_with_admin() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        // Send as ADDR1 to show admin is different
        let info = mock_info(ADDR1, &vec![]);

        // Create a message where ADDR2 will be an admin
        // Have to use .to_string() method
        let msg = InstantiateMsg {
            admin: Some(ADDR2.to_string()),
        };
        // Unwrap to assert success
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        // Assert admin is ADDR2 instead
        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR2),]
        );
    }
}
