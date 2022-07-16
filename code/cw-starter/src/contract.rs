#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// 06 Instantiate
// - // use cw2::set_contract_version;
// + use cw2::set_contract_version;
use cw2::set_contract_version;
// 06 Instantiate
// + use crate::state::{Config, CONFIG};
// 09 Execute 1
// - use crate::state::{Config, CONFIG};
// + use crate::state::{Config, CONFIG, Poll, POLLS};
// 10 Execute 2
// - use crate::state::{Config, CONFIG, Poll, POLLS};
// + use crate::state::{Config, Poll, CONFIG, POLLS, Ballot, BALLOTS};
use crate::state::{Ballot, Config, Poll, BALLOTS, CONFIG, POLLS};

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

// 09 Execute 1
// - _deps: DepsMut,
// - _env: Env,
// - _info: MessageInfo,
// - _msg: ExecuteMsg,
// + deps: DepsMut,
// + env: Env,
// + info: MessageInfo,
// + msg: ExecuteMsg,
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // 09 Execute 1
    // - unimplemented!()
    // + match msg {
    // +     ExecuteMsg::CreatePoll {
    // +         poll_id,
    // +         question,
    // +         options,
    // +     } => execute_create_poll(deps, env, info, poll_id, question, options),
    // +     ExecuteMsg::Vote { poll_id, vote } => unimplemented!(),
    // + }
    match msg {
        ExecuteMsg::CreatePoll {
            poll_id,
            question,
            options,
        } => execute_create_poll(deps, env, info, poll_id, question, options),
        // 10 Execute 2
        // - ExecuteMsg::Vote { poll_id, vote } => unimplemented!(),
        // + ExecuteMsg::Vote { poll_id, vote } => execute_vote(deps, env, info, poll_id, vote),
        ExecuteMsg::Vote { poll_id, vote } => execute_vote(deps, env, info, poll_id, vote),
    }
}

// 09 Execute 1
// + fn execute_create_poll(
// +     deps: DepsMut,
// +     _env: Env,
// +     info: MessageInfo,
// +     poll_id: String,
// +     question: String,
// +     options: Vec<String>,
// + ) -> Result<Response, ContractError> {
// +     if options.len() > 10 {
// +         return Err(ContractError::TooManyOptions {});
// +     }
// +
// +     let mut opts: Vec<(String, u64)> = vec![];
// +     for option in options {
// +         opts.push((option, 0));
// +     }
// +
// +     let poll = Poll {
// +         creator: info.sender,
// +         question,
// +         options: opts
// +     };
// +
// +     POLLS.save(deps.storage, poll_id, &poll)?;
// +
// +     Ok(Response::new())
// + }
fn execute_create_poll(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {
    if options.len() > 10 {
        return Err(ContractError::TooManyOptions {});
    }

    let mut opts: Vec<(String, u64)> = vec![];
    for option in options {
        opts.push((option, 0));
    }

    let poll = Poll {
        creator: info.sender,
        question,
        options: opts,
    };

    POLLS.save(deps.storage, poll_id, &poll)?;

    Ok(Response::new())
}

// 10 Execute 2
// + fn execute_vote(
// +     deps: DepsMut,
// +     _env: Env,
// +     info: MessageInfo,
// +     poll_id: String,
// +     vote: String,
// + ) -> Result<Response, ContractError> {
// +     let poll = POLLS.may_load(deps.storage, poll_id.clone())?;
// +
// +     match poll {
// +         Some(mut poll) => { // The poll exists
// +
// +             BALLOTS.update(
// +                 deps.storage,
// +                 (info.sender, poll_id.clone()),
// +                 |ballot| -> StdResult<Ballot> {
// +                     match ballot {
// +                         Some(ballot) => {
// +                             // We need to revoke their old vote
// +                             // Find the position
// +                             let position_of_old_vote = poll
// +                                 .options
// +                                 .iter()
// +                                 .position(|option| option.0 == ballot.option)
// +                                 .unwrap();
// +                             // Decrement by 1
// +                             poll.options[position_of_old_vote].1 -= 1;
// +                             // Update the ballot
// +                             Ok(Ballot {
// +                                 option: vote.clone(),
// +                             })
// +                         }
// +                         None => {
// +                             // Simply add the ballot
// +                             Ok(Ballot {
// +                                 option: vote.clone(),
// +                             })
// +                         }
// +                     }
// +                 },
// +            )?;
// +
// +             // Find the position of the new vote option and increment it by 1
// +             let position = poll.options.iter().position(|option| option.0 == vote);
// +             if position.is_none() {
// +                 return Err(ContractError::Unauthorized {});
// +             }
// +             let position = position.unwrap();
// +             poll.options[position].1 += 1;
// +
// +             // Save the update
// +             POLLS.save(deps.storage, poll_id, &poll)?;
// +             Ok(Response::new())
// +         }
// +         None => Err(ContractError::Unauthorized {}), // The poll does not exist so we just error
// +     }
// + }
fn execute_vote(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    poll_id: String,
    vote: String,
) -> Result<Response, ContractError> {
    let poll = POLLS.may_load(deps.storage, poll_id.clone())?;

    match poll {
        Some(mut poll) => {
            // The poll exists
            BALLOTS.update(
                deps.storage,
                (info.sender, poll_id.clone()),
                |ballot| -> StdResult<Ballot> {
                    match ballot {
                        Some(ballot) => {
                            // We need to revoke their old vote
                            // Find the position
                            let position_of_old_vote = poll
                                .options
                                .iter()
                                .position(|option| option.0 == ballot.option)
                                .unwrap();
                            // Decrement by 1
                            poll.options[position_of_old_vote].1 -= 1;
                            // Update the ballot
                            Ok(Ballot {
                                option: vote.clone(),
                            })
                        }
                        None => {
                            // Simply add the ballot
                            Ok(Ballot {
                                option: vote.clone(),
                            })
                        }
                    }
                },
            )?;

            // Find the position of the new vote option and increment it by 1
            let position = poll.options.iter().position(|option| option.0 == vote);
            if position.is_none() {
                return Err(ContractError::Unauthorized {});
            }
            let position = position.unwrap();
            poll.options[position].1 += 1;

            // Save the update
            POLLS.save(deps.storage, poll_id, &poll)?;
            Ok(Response::new())
        }
        None => Err(ContractError::Unauthorized {}), // The poll does not exist so we just error
    }
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

    // 11 Execute Tests
    // - use crate::contract::instantiate; // the contract instantiate function
    // - use crate::msg::InstantiateMsg;
    // + use crate::contract::{instantiate, execute};
    // + use crate::msg::{InstantiateMsg, ExecuteMsg};
    use crate::contract::{execute, instantiate};
    use crate::msg::{ExecuteMsg, InstantiateMsg};
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

    // 11 Execute Tests
    // + #[test]
    // + fn test_execute_create_poll_valid() {
    // +     let mut deps = mock_dependencies();
    // +     let env = mock_env();
    // +     let info = mock_info(ADDR1, &vec![]);
    // +     // Instantiate the contract
    // +     let msg = InstantiateMsg { admin: None };
    // +     let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // New execute msg
    // +     let msg = ExecuteMsg::CreatePoll {
    // +         poll_id: "some_id".to_string(),
    // +         question: "What's your favourite Cosmos coin?".to_string(),
    // +         options: vec![
    // +             "Cosmos Hub".to_string(),
    // +             "Juno".to_string(),
    // +             "Osmosis".to_string(),
    // +         ],
    // +     };
    // +
    // +     // Unwrap to assert success
    // +     let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    // + }
    #[test]
    fn test_execute_create_poll_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // New execute msg
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };

        // Unwrap to assert success
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }

    // 11 Execute Tests
    // + #[test]
    // + fn test_execute_create_poll_invalid() {
    // +     let mut deps = mock_dependencies();
    // +     let env = mock_env();
    // +     let info = mock_info(ADDR1, &vec![]);
    // +     // Instantiate the contract
    // +     let msg = InstantiateMsg { admin: None };
    // +     let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     let msg = ExecuteMsg::CreatePoll {
    // +         poll_id: "some_id".to_string(),
    // +         question: "What's your favourite number?".to_string(),
    // +         options: vec![
    // +             "1".to_string(),
    // +             "2".to_string(),
    // +             "3".to_string(),
    // +             "4".to_string(),
    // +             "5".to_string(),
    // +             "6".to_string(),
    // +             "7".to_string(),
    // +             "8".to_string(),
    // +             "9".to_string(),
    // +             "10".to_string(),
    // +             "11".to_string(),
    // +         ],
    // +     };
    // +
    // +     // Unwrap error to assert failure
    // +     let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    // + }
    #[test]
    fn test_execute_create_poll_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite number?".to_string(),
            options: vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
                "10".to_string(),
                "11".to_string(),
            ],
        };

        // Unwrap error to assert failure
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }

    // 11 Execute Tests
    // + #[test]
    // + fn test_execute_vote_valid() {
    // +     let mut deps = mock_dependencies();
    // +     let env = mock_env();
    // +     let info = mock_info(ADDR1, &vec![]);
    // +     // Instantiate the contract
    // +     let msg = InstantiateMsg { admin: None };
    // +     let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // Create the poll
    // +     let msg = ExecuteMsg::CreatePoll {
    // +         poll_id: "some_id".to_string(),
    // +         question: "What's your favourite Cosmos coin?".to_string(),
    // +         options: vec![
    // +             "Cosmos Hub".to_string(),
    // +             "Juno".to_string(),
    // +             "Osmosis".to_string(),
    // +         ],
    // +     };
    // +     let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // Create the vote, first time voting
    // +     let msg = ExecuteMsg::Vote {
    // +         poll_id: "some_id".to_string(),
    // +         vote: "Juno".to_string(),
    // +     };
    // +     let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // Change the vote
    // +     let msg = ExecuteMsg::Vote {
    // +         poll_id: "some_id".to_string(),
    // +         vote: "Osmosis".to_string(),
    // +     };
    // +     let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    // + }
    #[test]
    fn test_execute_vote_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create the poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create the vote, first time voting
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Juno".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Change the vote
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Osmosis".to_string(),
        };
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }

    // 11 Execute Tests
    // + #[test]
    // + fn test_execute_vote_invalid() {
    // +     let mut deps = mock_dependencies();
    // +     let env = mock_env();
    // +     let info = mock_info(ADDR1, &vec![]);
    // +     // Instantiate the contract
    // +     let msg = InstantiateMsg { admin: None };
    // +     let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // Create the vote, some_id poll is not created yet.
    // +     let msg = ExecuteMsg::Vote {
    // +         poll_id: "some_id".to_string(),
    // +         vote: "Juno".to_string(),
    // +     };
    // +     // Unwrap to assert error
    // +     let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    // +
    // +     // Create the poll
    // +     let msg = ExecuteMsg::CreatePoll {
    // +         poll_id: "some_id".to_string(),
    // +         question: "What's your favourite Cosmos coin?".to_string(),
    // +         options: vec![
    // +             "Cosmos Hub".to_string(),
    // +             "Juno".to_string(),
    // +             "Osmosis".to_string(),
    // +         ],
    // +     };
    // +     let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // Vote on a now existing poll but the option "DVPN" does not exist
    // +     let msg = ExecuteMsg::Vote {
    // +         poll_id: "some_id".to_string(),
    // +         vote: "DVPN".to_string(),
    // +     };
    // +     let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    // + }
    #[test]
    fn test_execute_vote_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create the vote, some_id poll is not created yet.
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Juno".to_string(),
        };
        // Unwrap to assert error
        let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        // Create the poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Vote on a now existing poll but the option "DVPN" does not exist
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "DVPN".to_string(),
        };
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }
}
