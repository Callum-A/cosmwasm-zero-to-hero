#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
// 13 Query
// - use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// + use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Order, to_binary};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
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
// 13 Query
// - use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
// + use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, AllPollsResponse, PollResponse, VoteResponse};
use crate::msg::{
    AllPollsResponse, ExecuteMsg, InstantiateMsg, PollResponse, QueryMsg, VoteResponse,
};

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
    let admin = msg.admin.unwrap_or_else(|| info.sender.to_string());
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

    // 15 Spring Cleaning
    // - POLLS.save(deps.storage, poll_id, &poll)?;
    // + POLLS.save(deps.storage, &poll_id, &poll)?;
    POLLS.save(deps.storage, &poll_id, &poll)?;

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
    // 15 Spring Cleaning
    // - let poll = POLLS.may_load(deps.storage, poll_id.clone())?;
    // + let poll = POLLS.may_load(deps.storage, &poll_id)?;
    let poll = POLLS.may_load(deps.storage, &poll_id)?;

    match poll {
        Some(mut poll) => {
            // The poll exists
            BALLOTS.update(
                deps.storage,
                // 15 Spring Cleaning
                // - (info.sender, poll_id.clone()),
                // + (info.sender, &poll_id)
                (info.sender, &poll_id),
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
            // 15 Spring Cleaning
            // - POLLS.save(deps.storage, poll_id, &poll)?;
            // + POLLS.save(deps.storage, &poll_id, &poll)?;
            POLLS.save(deps.storage, &poll_id, &poll)?;
            Ok(Response::new())
        }
        None => Err(ContractError::Unauthorized {}), // The poll does not exist so we just error
    }
}

// 13 Query
// - #[cfg_attr(not(feature = "library"), entry_point)]
// - pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
// -     unimplemented!()
// - }
// + #[cfg_attr(not(feature = "library"), entry_point)]
// + pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
// +     match msg {
// +         QueryMsg::AllPolls {} => query_all_polls(deps, env),
// +         QueryMsg::Poll { poll_id } => query_poll(deps, env, poll_id),
// +         QueryMsg::Vote { address, poll_id } => query_vote(deps, env, address, poll_id),
// +     }
// + }
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllPolls {} => query_all_polls(deps, env),
        QueryMsg::Poll { poll_id } => query_poll(deps, env, poll_id),
        QueryMsg::Vote { address, poll_id } => query_vote(deps, env, address, poll_id),
    }
}

// 13 Query
// + fn query_all_polls(deps: Deps, _env: Env) -> StdResult<Binary> {
// +     let polls = POLLS
// +         .range(deps.storage, None, None, Order::Ascending)
// +         .map(|p| Ok(p?.1))
// +         .collect::<StdResult<Vec<_>>>()?;
// +
// +    to_binary(&AllPollsResponse { polls })
// + }
fn query_all_polls(deps: Deps, _env: Env) -> StdResult<Binary> {
    let polls = POLLS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&AllPollsResponse { polls })
}

// 13 Query
// + fn query_poll(deps: Deps, _env: Env, poll_id: String) -> StdResult<Binary> {
// +     let poll = POLLS.may_load(deps.storage, poll_id)?;
// +     to_binary(&PollResponse { poll })
// + }
fn query_poll(deps: Deps, _env: Env, poll_id: String) -> StdResult<Binary> {
    // 15 Spring Cleaning
    // - let poll = POLLS.may_load(deps.storage, poll_id)?;
    // + let poll = POLLS.may_load(deps.storage, &poll_id)?;
    let poll = POLLS.may_load(deps.storage, &poll_id)?;
    to_binary(&PollResponse { poll })
}

// 13 Query
// + fn query_vote(deps: Deps, _env: Env, address: String, poll_id: String) -> StdResult<Binary> {
// +     let validated_address = deps.api.addr_validate(&address).unwrap();
// +     let vote = BALLOTS.may_load(deps.storage, (validated_address, poll_id))?;
// +
// +     to_binary(&VoteResponse { vote })
// + }
fn query_vote(deps: Deps, _env: Env, address: String, poll_id: String) -> StdResult<Binary> {
    let validated_address = deps.api.addr_validate(&address).unwrap();
    // 15 Spring Cleaning
    // - let vote = BALLOTS.may_load(deps.storage, (validated_address, poll_id))?;
    // + let vote = BALLOTS.may_load(deps.storage, (validated_address, &poll_id))?;
    let vote = BALLOTS.may_load(deps.storage, (validated_address, &poll_id))?;

    to_binary(&VoteResponse { vote })
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

    // 14 Query Tests
    // - use crate::contract::{execute, instantiate};
    // - use crate::msg::{ExecuteMsg, InstantiateMsg};
    // + use crate::contract::{execute, instantiate, query};
    // + use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, AllPollsResponse, PollResponse, VoteResponse};
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{
        AllPollsResponse, ExecuteMsg, InstantiateMsg, PollResponse, QueryMsg, VoteResponse,
    };
    // 14 Query Tests
    // - use cosmwasm_std::attr;
    // + use cosmwasm_std::{attr, from_binary};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary}; // helper to construct an attribute e.g. ("action", "instantiate") // mock functions to mock an environment, message info, dependencies // our instantate method

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
        let info = mock_info(ADDR1, &[]);

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
        let info = mock_info(ADDR1, &[]);

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
        let info = mock_info(ADDR1, &[]);
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
        let info = mock_info(ADDR1, &[]);
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
        let info = mock_info(ADDR1, &[]);
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
        let info = mock_info(ADDR1, &[]);
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

    // 14 Query Tests
    // + #[test]
    // + fn test_query_all_polls() {
    // +     let mut deps = mock_dependencies();
    // +     let env = mock_env();
    // +     let info = mock_info(ADDR1, &vec![]);
    // +     // Instantiate the contract
    // +     let msg = InstantiateMsg { admin: None };
    // +     let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // Create a poll
    // +     let msg = ExecuteMsg::CreatePoll {
    // +         poll_id: "some_id_1".to_string(),
    // +         question: "What's your favourite Cosmos coin?".to_string(),
    // +         options: vec![
    // +             "Cosmos Hub".to_string(),
    // +             "Juno".to_string(),
    // +             "Osmosis".to_string(),
    // +         ],
    // +     };
    // +     let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // Create a second poll
    // +     let msg = ExecuteMsg::CreatePoll {
    // +         poll_id: "some_id_2".to_string(),
    // +         question: "What's your colour?".to_string(),
    // +         options: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
    // +     };
    // +     let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    // +
    // +     // Query
    // +     let msg = QueryMsg::AllPolls {};
    // +     let bin = query(deps.as_ref(), env, msg).unwrap();
    // +     let res: AllPollsResponse = from_binary(&bin).unwrap();
    // +     assert_eq!(res.polls.len(), 2);
    // + }
    #[test]
    fn test_query_all_polls() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_1".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a second poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_2".to_string(),
            question: "What's your colour?".to_string(),
            options: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
        };
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Query
        let msg = QueryMsg::AllPolls {};
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: AllPollsResponse = from_binary(&bin).unwrap();
        assert_eq!(res.polls.len(), 2);
    }

    // 14 Query Tests
    // + #[test]
    // + fn test_query_poll() {
    // +     let mut deps = mock_dependencies();
    // +     let env = mock_env();
    // +     let info = mock_info(ADDR1, &vec![]);
    // +     // Instantiate the contract
    // +     let msg = InstantiateMsg { admin: None };
    // +     let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // Create a poll
    // +     let msg = ExecuteMsg::CreatePoll {
    // +         poll_id: "some_id_1".to_string(),
    // +         question: "What's your favourite Cosmos coin?".to_string(),
    // +         options: vec![
    // +             "Cosmos Hub".to_string(),
    // +             "Juno".to_string(),
    // +             "Osmosis".to_string(),
    // +         ],
    // +     };
    // +     let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // Query for the poll that exists
    // +     let msg = QueryMsg::Poll {
    // +         poll_id: "some_id_1".to_string(),
    // +     };
    // +     let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    // +     let res: PollResponse = from_binary(&bin).unwrap();
    // +     // Expect a poll
    // +     assert!(res.poll.is_some());
    // +
    // +     // Query for the poll that does not exists
    // +     let msg = QueryMsg::Poll {
    // +         poll_id: "some_id_not_exist".to_string(),
    // +     };
    // +     let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    // +     let res: PollResponse = from_binary(&bin).unwrap();
    // +     // Expect none
    // +     assert!(res.poll.is_none());
    // + }
    #[test]
    fn test_query_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_1".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Query for the poll that exists
        let msg = QueryMsg::Poll {
            poll_id: "some_id_1".to_string(),
        };
        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: PollResponse = from_binary(&bin).unwrap();
        // Expect a poll
        assert!(res.poll.is_some());

        // Query for the poll that does not exists
        let msg = QueryMsg::Poll {
            poll_id: "some_id_not_exist".to_string(),
        };
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: PollResponse = from_binary(&bin).unwrap();
        // Expect none
        assert!(res.poll.is_none());
    }

    // 14 Query Tests
    // + #[test]
    // + fn test_query_vote() {
    // +     let mut deps = mock_dependencies();
    // +     let env = mock_env();
    // +     let info = mock_info(ADDR1, &vec![]);
    // +     // Instantiate the contract
    // +     let msg = InstantiateMsg { admin: None };
    // +     let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // Create a poll
    // +     let msg = ExecuteMsg::CreatePoll {
    // +         poll_id: "some_id_1".to_string(),
    // +         question: "What's your favourite Cosmos coin?".to_string(),
    // +         options: vec![
    // +             "Cosmos Hub".to_string(),
    // +             "Juno".to_string(),
    // +             "Osmosis".to_string(),
    // +         ],
    // +     };
    // +     let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // Create a vote
    // +     let msg = ExecuteMsg::Vote {
    // +         poll_id: "some_id_1".to_string(),
    // +         vote: "Juno".to_string(),
    // +     };
    // +     let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // +
    // +     // Query for a vote that exists
    // +     let msg = QueryMsg::Vote {
    // +         poll_id: "some_id_1".to_string(),
    // +         address: ADDR1.to_string(),
    // +     };
    // +     let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    // +     let res: VoteResponse = from_binary(&bin).unwrap();
    // +     // Expect the vote to exist
    // +     assert!(res.vote.is_some());
    // +
    // +     // Query for a vote that does not exists
    // +     let msg = QueryMsg::Vote {
    // +         poll_id: "some_id_2".to_string(),
    // +         address: ADDR2.to_string(),
    // +     };
    // +     let bin = query(deps.as_ref(), env, msg).unwrap();
    // +     let res: VoteResponse = from_binary(&bin).unwrap();
    // +     // Expect the vote to not exist
    // +     assert!(res.vote.is_none());
    // + }
    #[test]
    fn test_query_vote() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_1".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a vote
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id_1".to_string(),
            vote: "Juno".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Query for a vote that exists
        let msg = QueryMsg::Vote {
            poll_id: "some_id_1".to_string(),
            address: ADDR1.to_string(),
        };
        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: VoteResponse = from_binary(&bin).unwrap();
        // Expect the vote to exist
        assert!(res.vote.is_some());

        // Query for a vote that does not exists
        let msg = QueryMsg::Vote {
            poll_id: "some_id_2".to_string(),
            address: ADDR2.to_string(),
        };
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: VoteResponse = from_binary(&bin).unwrap();
        // Expect the vote to not exist
        assert!(res.vote.is_none());
    }
}
